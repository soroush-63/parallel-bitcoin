use bitcoin::hash_types::BlockHash;
use bitcoin::util::hash::Sha256dHash;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelBlock {
    pub chain_id: u8,
    pub version: u32,
    pub prev_meta_hash: BlockHash,
    pub merkle_root: Sha256dHash,
    pub timestamp: u64,
    pub bits: u32,           // difficulty target in compact form
    pub nonce: u32,
    pub data: Vec<u8>,       // بعداً می‌تونی تراکنش‌ها رو اینجا بذاری
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaBlock {
    pub meta_version: u32,
    pub previous_meta_hash: BlockHash,
    pub timemetastamp: u64,
    pub block_hashes: Vec<BlockHash>,  // هش بلاک‌های موازی
    pub meta_merkle_root: Sha256dHash,
}

pub struct ChainParams {
    pub genesis_hash: BlockHash,
    pub target_spacing: u64,      // فاصله زمانی هدف بین بلاک‌ها (ثانیه)
    pub retarget_period: u64,     // هر چند بلاک سختی تنظیم بشه
    pub pow_limit: u32,           // حداکثر سختی
}

impl ChainParams {
    pub fn new_testnet() -> Self {
        ChainParams {
            genesis_hash: BlockHash::all_zeros(), // بعداً واقعی کن
            target_spacing: 120,                  // ۲ دقیقه
            retarget_period: 2016,
            pow_limit: 0x1d00ffff,                // سختی پایین برای تست
        }
    }
}

pub fn genesis_block(chain_id: u8) -> ParallelBlock {
    let params = ChainParams::new_testnet();
    ParallelBlock {
        chain_id,
        version: 1,
        prev_meta_hash: BlockHash::all_zeros(),
        merkle_root: Sha256dHash::all_zeros(),
        timestamp: Utc::now().timestamp() as u64,
        bits: params.pow_limit,
        nonce: 0,
        data: vec![],
    }
}