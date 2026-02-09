use sha2::{Digest, Sha256};

// ======= BlockHeader =======
#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_meta_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub difficulty_target: u32,
    pub nonce: u64,
}

impl BlockHeader {
    /// هش بلوک، برای PoW و MetaBlock
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.version.to_le_bytes());
        hasher.update(self.prev_meta_hash);
        hasher.update(self.merkle_root);
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.difficulty_target.to_le_bytes());
        hasher.update(self.nonce.to_le_bytes());
        hasher.finalize().into()
    }
}

// ======= Block =======
#[derive(Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub body: Vec<u8>, // تراکنش‌ها یا داده‌های بلاک
}

impl Default for Block {
    fn default() -> Self {
        Block {
            header: BlockHeader {
                version: 1,
                prev_meta_hash: [0u8; 32],
                merkle_root: [0u8; 32],
                timestamp: 0,
                difficulty_target: 2,
                nonce: 0,
            },
            body: vec![],
        }
    }
}
