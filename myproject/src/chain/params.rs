use bitcoin::blockdata::block::BlockHeader;
use bitcoin::hash_types::{BlockHash, TxMerkleNode};
use bitcoin::hashes::sha256d;
use bitcoin::Amount;
use bitcoin::Script;
use std::time::{SystemTime, UNIX_EPOCH};

/// پارامترهای شبکه موازی
pub struct ChainParams {
    pub network_name: String,
    pub target_spacing: u64,      // زمان بین بلوک‌ها (ثانیه)
    pub difficulty_adjustment_interval: u32, // هر چند بلوک سختی عوض شود
    pub subsidy_halving_interval: u32, // هاوینگ
    pub max_block_size: usize,    // سایز بلوک
    pub pow_limit: [u8; 32],      // حداقل سختی
}

impl ChainParams {
    pub fn new_testnet() -> Self {
        Self {
            network_name: "parallel_testnet".to_string(),
            target_spacing: 120, // ۲ دقیقه
            difficulty_adjustment_interval: 2016,
            subsidy_halving_interval: 210000,
            max_block_size: 32 * 1024 * 1024, // 32 مگابایت
            pow_limit: [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ],
        }
    }
    
    /// ساخت بلوک جنسیس برای یک زنجیره
    pub fn create_genesis_block(chain_id: u8) -> crate::chain::block::ParallelBlock {
        let timestamp = 1715000000; // یک timestamp ثابت
        
        // ساخت هدر ساده
        let mut header = BlockHeader {
            version: 1,
            prev_blockhash: BlockHash::from_slice(&[0u8; 32]).unwrap(), // جنسیس هیچ والد نداره
            merkle_root: TxMerkleNode::from_raw_hash(sha256d::Hash::hash("genesis".as_bytes())),
            time: timestamp,
            bits: 0x1f0fffff, // سختی پایین
            nonce: 0,
        };
        
        // پیدا کردن نانس معتبر
        for nonce in 0..u32::MAX {
            header.nonce = nonce;
            // TODO: بررسی PoW - فعلاً نادیده بگیر
            break; // برای تست سریع
        }
        
        crate::chain::block::ParallelBlock::new(header, chain_id)
    }
    
    pub fn pow_limit(&self) -> [u8; 32] {
        self.pow_limit
    }
}