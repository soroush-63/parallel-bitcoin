use bitcoin::hash_types::BlockHash;
use std::time::Instant;

use crate::chain::{ParallelBlock, MetaBlock, ChainParams};

pub struct SimpleMiner {
    params: ChainParams,
}

pub struct ParallelMiner {
    params: ChainParams,
}

impl SimpleMiner {
    pub fn new(params: ChainParams) -> Self {
        SimpleMiner { params }
    }

    pub fn mine_block(&self, prev_meta_hash: BlockHash, chain_id: u8) -> ParallelBlock {
        let mut block = crate::chain::genesis_block(chain_id);
        block.prev_meta_hash = prev_meta_hash;
        block.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // ماینینگ ساده – در آینده موازی می‌کنیم
        block.nonce = 0;
        block
    }
}

impl ParallelMiner {
    pub fn new(params: ChainParams) -> Self {
        ParallelMiner { params }
    }

    pub fn mine_parallel_blocks(&self, prev_meta_hash: BlockHash) -> (ParallelBlock, ParallelBlock) {
        let start = Instant::now();

        // اینجا بعداً با rayon موازی واقعی می‌کنیم
        let block_a = SimpleMiner::new(self.params.clone()).mine_block(prev_meta_hash, 0);
        let block_b = SimpleMiner::new(self.params.clone()).mine_block(prev_meta_hash, 1);

        println!("ماین دو بلاک موازی در {:?} طول کشید", start.elapsed());
        (block_a, block_b)
    }

    pub fn create_meta_block(&self, block_a: &ParallelBlock, block_b: &ParallelBlock) -> MetaBlock {
        let block_hashes = vec![block_a.prev_meta_hash, block_b.prev_meta_hash];
        let meta_merkle_root = Sha256dHash::all_zeros(); // بعداً واقعی محاسبه کن

        MetaBlock {
            meta_version: 1,
            previous_meta_hash: block_a.prev_meta_hash,
            timemetastamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            block_hashes,
            meta_merkle_root,
        }
    }
}