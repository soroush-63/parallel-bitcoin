use crate::chain::block::{ParallelBlock, MetaBlock};
use crate::chain::params::ChainParams;
use bitcoin::blockdata::block::{BlockHeader, Block};
use bitcoin::blockdata::transaction::{Transaction, TxOut};
use bitcoin::blockdata::script::Script;
use bitcoin::hash_types::BlockHash;
use bitcoin::hashes::{sha256d, Hash};
use bitcoin::util::amount::Amount;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// ماینر ساده برای یک زنجیره
pub struct SimpleMiner {
    chain_id: u8,
    params: Arc<ChainParams>,
    mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl SimpleMiner {
    pub fn new(chain_id: u8, params: Arc<ChainParams>) -> Self {
        Self {
            chain_id,
            params,
            mempool: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// اضافه کردن تراکنش به ممپول
    pub fn add_transaction(&self, tx: Transaction) {
        let mut mempool = self.mempool.lock().unwrap();
        mempool.push(tx);
    }
    
    /// استخراج یک بلوک موازی
    pub fn mine_block(&self, prev_meta_hash: BlockHash) -> ParallelBlock {
        println!("ماینینگ زنجیره {} شروع شد...", self.chain_id);
        
        // گرفتن تراکنش‌ها از ممپول
        let transactions = self.get_transactions_from_mempool();
        
        // ساخت هدر بلوک
        let mut header = BlockHeader {
            version: 1,
            prev_blockhash: prev_meta_hash, // اشاره به متابلوک قبلی
            merkle_root: self.calculate_merkle_root(&transactions),
            time: current_timestamp(),
            bits: 0x1f0fffff, // سختی پایین برای تست
            nonce: 0,
        };
        
        // حل اثبات کار
        self.solve_pow(&mut header);
        
        println!("بلوک {} استخراج شد! نانس: {}", self.chain_id, header.nonce);
        
        ParallelBlock::new(header, transactions, self.chain_id)
    }
    
    /// حل اثبات کار ساده
    fn solve_pow(&self, header: &mut BlockHeader) {
        let target = self.params.pow_limit();
        
        for nonce in 0..u32::MAX {
            header.nonce = nonce;
            
            // بررسی معتبر بودن PoW
            if header.validate_pow(&target).is_ok() {
                return;
            }
            
            // نمایش پیشرفت
            if nonce % 1_000_000 == 0 {
                print!("\r  {}M تلاش...", nonce / 1_000_000);
            }
        }
        
        panic!("نانس پیدا نشد!");
    }
    
    fn get_transactions_from_mempool(&self) -> Vec<Transaction> {
        let mut mempool = self.mempool.lock().unwrap();
        
        // برای تست: ساخت تراکنش ساده
        if mempool.is_empty() {
            // یک تراکنش تستی
            let test_tx = Transaction {
                version: 1,
                lock_time: 0,
                input: vec![],
                output: vec![TxOut {
                    value: Amount::from_sat(1000),
                    script_pubkey: Script::new(),
                }],
            };
            mempool.push(test_tx);
        }
        
        // حداکثر 1000 تراکنش
        mempool.drain(..std::cmp::min(mempool.len(), 1000)).collect()
    }
    
    fn calculate_merkle_root(&self, transactions: &[Transaction]) -> sha256d::Hash {
        if transactions.is_empty() {
            return sha256d::Hash::hash(b"empty");
        }
        
        // ساده‌سازی: فقط هش اولین تراکنش
        sha256d::Hash::hash(&transactions[0].txid().as_hash().to_byte_array())
    }
}

/// ماینر موازی که دو بلوک همزمان استخراج می‌کند
pub struct ParallelMiner {
    miner_a: Arc<SimpleMiner>,
    miner_b: Arc<SimpleMiner>,
    params: Arc<ChainParams>,
}

impl ParallelMiner {
    pub fn new(params: Arc<ChainParams>) -> Self {
        Self {
            miner_a: Arc::new(SimpleMiner::new(0, Arc::clone(&params))),
            miner_b: Arc::new(SimpleMiner::new(1, Arc::clone(&params))),
            params,
        }
    }
    
    /// استخراج همزمان دو بلوک موازی
    pub fn mine_parallel_blocks(&self, prev_meta_hash: BlockHash) -> (ParallelBlock, ParallelBlock) {
        println!("شروع ماینینگ موازی...");
        
        // کپی از prev_meta_hash برای هر دو ترد
        let prev_hash_clone = prev_meta_hash;
        
        // استخراج موازی در دو ترد
        let miner_a = Arc::clone(&self.miner_a);
        let handle_a = thread::spawn(move || {
            miner_a.mine_block(prev_hash_clone)
        });
        
        let miner_b = Arc::clone(&self.miner_b);
        let handle_b = thread::spawn(move || {
            miner_b.mine_block(prev_meta_hash)
        });
        
        let block_a = handle_a.join().expect("Thread A failed");
        let block_b = handle_b.join().expect("Thread B failed");
        
        (block_a, block_b)
    }
    
    /// ساخت متابلوک از دو بلوک موازی
    pub fn create_meta_block(&self, block_a: &ParallelBlock, block_b: &ParallelBlock, prev_meta_hash: BlockHash) -> MetaBlock {
        MetaBlock::new(
            prev_meta_hash,
            block_a.hash(),
            block_b.hash(),
        )
    }
}

/// تابع کمکی برای timestamp
fn current_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}