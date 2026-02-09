use std::thread;
use crate::chain::block::{Block, BlockHeader};
use crate::metablock::*;

/// استخراج بلوک‌های موازی با PoW واقعی
pub fn mine_parallel_blocks(headers: Vec<BlockHeader>, difficulty: usize) -> Vec<Block> {
    let mut handles = vec![];

    for header in headers {
        handles.push(thread::spawn(move || {
            let mut h = header;
            loop {
                let hash = h.hash();
                if hash.iter().take(difficulty).all(|b| *b == 0) {
                    break;
                }
                h.nonce += 1;
            }
            Block { header: h, body: Default::default() }
        }));
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// استخراج بلوک‌های موازی و ساخت MetaBlock از آن‌ها
pub fn mine_blocks_with_metablock(prev_meta_hash: [u8; 32], headers: Vec<BlockHeader>, difficulty: usize) -> (Vec<Block>, MetaBlock) {
    // تنظیم prev_meta_hash برای تمام بلوک‌ها
    let headers: Vec<BlockHeader> = headers.into_iter().map(|mut h| {
        h.prev_meta_hash = prev_meta_hash;
        h
    }).collect();

    // استخراج بلوک‌های موازی
    let blocks = mine_parallel_blocks(headers, difficulty);

    // ساخت Merkle root MetaBlock
    let meta_root = meta_merkle_from_headers(
        &blocks.iter().map(|b| b.header.clone()).collect::<Vec<_>>()
    );

    // ساخت و استخراج MetaBlock واقعی
    let meta = MetaBlock {
        version: 1,
        prev_meta_hash,
        meta_merkle_root: meta_root,
        timestamp: chrono::Utc::now().timestamp() as u64,
        nonce: 0,
    }.mine(difficulty);

    (blocks, meta)
}
