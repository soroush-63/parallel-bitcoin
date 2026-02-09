use sha2::{Digest, Sha256};
use crate::chain::block::BlockHeader;

pub type Hash = [u8; 32];

#[derive(Clone)]
pub struct MetaBlock {
    pub version: u32,
    pub prev_meta_hash: Hash,
    pub meta_merkle_root: Hash,
    pub timestamp: u64,
    pub nonce: u64,
}

impl MetaBlock {
    /// PoW واقعی برای MetaBlock
    pub fn mine(mut self, difficulty: usize) -> Self {
        loop {
            let hash = self.hash();
            if hash.iter().take(difficulty).all(|b| *b == 0) {
                break;
            }
            self.nonce += 1;
        }
        self
    }

    /// هش MetaBlock
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.version.to_le_bytes());
        hasher.update(self.prev_meta_hash);
        hasher.update(self.meta_merkle_root);
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.nonce.to_le_bytes());
        hasher.finalize().into()
    }
}

/// تولید Merkle root ساده از headerهای بلوک‌های موازی
pub fn meta_merkle_from_headers(headers: &[BlockHeader]) -> Hash {
    let mut hasher = Sha256::new();
    for h in headers {
        hasher.update(h.hash());
    }
    hasher.finalize().into()
}
