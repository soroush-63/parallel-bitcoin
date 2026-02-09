use crate::chain::block::BlockHeader;
use crate::miner::mod::mine_blocks_with_metablock;
use crate::metablock::*;
use chrono::Utc;

fn now() -> u64 {
    Utc::now().timestamp() as u64
}

fn main() {
    println!("🚀 Starting parallel blockchain with MetaBlock...");

    // شروع prev_meta_hash صفر
    let mut prev_meta_hash = [0u8; 32];

    // سختی PoW (برای تست سریع می‌تونی 1 یا 2 بذاری)
    let difficulty = 2;

    // تعداد سیکل‌ها (چند "بلاک موازی + MetaBlock")
    let cycles = 5;

    // داده‌های ساده برای دو بلوک موازی
    let base_headers = vec![
        BlockHeader {
            version: 1,
            prev_meta_hash,
            merkle_root: [1u8; 32],
            timestamp: now(),
            difficulty_target: difficulty as u32,
            nonce: 0,
        },
        BlockHeader {
            version: 1,
            prev_meta_hash,
            merkle_root: [2u8; 32],
            timestamp: now(),
            difficulty_target: difficulty as u32,
            nonce: 0,
        },
    ];

    for cycle in 0..cycles {
        println!("\n⛏ Cycle {}: Mining parallel blocks...", cycle + 1);

        // هر سیکل بلوک‌ها و MetaBlock استخراج می‌شوند
        let (blocks, meta) = mine_blocks_with_metablock(prev_meta_hash, base_headers.clone(), difficulty);

        for (i, block) in blocks.iter().enumerate() {
            println!("Block {} header nonce: {}", i + 1, block.header.nonce);
        }

        println!("✅ MetaBlock mined! Nonce: {}", meta.nonce);
        println!("Meta hash: {:x?}", meta.hash());

        // هش MetaBlock برای بلوک‌های بعدی استفاده می‌شود
        prev_meta_hash = meta.hash();
    }

    println!("\n🎉 Parallel blockchain with MetaBlock finished!");
}
