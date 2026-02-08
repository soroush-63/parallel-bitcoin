// src/chain/block.rs
use bitcoin::blockdata::block::BlockHeader;
use bitcoin::hash_types::BlockHash;
use bitcoin::hashes::Hash;

/// بلوک موازی با chain_id
#[derive(Debug, Clone)]
pub struct ParallelBlock {
    pub header: BlockHeader,
    pub chain_id: u8, // 0 یا 1
    // transactions رو موقتاً حذف می‌کنیم تا compile شه
}

impl ParallelBlock {
    pub fn new(header: BlockHeader, chain_id: u8) -> Self {
        Self {
            header,
            chain_id,
        }
    }
    
    pub fn hash(&self) -> BlockHash {
        self.header.block_hash()
    }
}

/// متابلوک برای اتصال دو زنجیره موازی
#[derive(Debug, Clone)]
pub struct MetaBlock {
    pub prev_meta_hash: BlockHash,
    pub block_a_hash: BlockHash,
    pub block_b_hash: BlockHash,
    pub timestamp: u32,
}

impl MetaBlock {
    pub fn new(prev_meta_hash: BlockHash, block_a_hash: BlockHash, block_b_hash: BlockHash) -> Self {
        Self {
            prev_meta_hash,
            block_a_hash,
            block_b_hash,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        }
    }
    
    pub fn hash(&self) -> BlockHash {
        let data = format!(
            "{}{}{}{}",
            self.prev_meta_hash,
            self.block_a_hash,
            self.block_b_hash,
            self.timestamp
        );
        bitcoin::hashes::sha256d::Hash::hash(data.as_bytes()).into()
    }
}
