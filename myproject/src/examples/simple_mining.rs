// examples/simple_mining.rs
use parallel_bitcoin::{run_mining_round, genesis_block};
use bitcoin::hash_types::BlockHash;

fn main() {
    println!("🚀 شروع تست ماینینگ موازی");
    println!("=".repeat(50));
    
    // بلوک‌های جنسیس
    println!("ساخت بلوک‌های جنسیس...");
    let genesis_a = genesis_block(0);
    let genesis_b = genesis_block(1);
    
    println!("بلوک جنسیس A: chain_id={}, hash={:?}", 
        genesis_a.chain_id, 
        genesis_a.hash()
    );
    println!("بلوک جنسیس B: chain_id={}, hash={:?}", 
        genesis_b.chain_id, 
        genesis_b.hash()
    );
    
    // ساخت متابلوک جنسیس
    use parallel_bitcoin::chain::block::MetaBlock;
    let genesis_meta = MetaBlock::new(
        BlockHash::all_zeros(),
        genesis_a.hash(),
        genesis_b.hash(),
    );
    
    println!("متابلوک جنسیس: hash={:?}", genesis_meta.hash());
    println!();
    
    // اجرای ۳ دوره ماینینگ
    let mut prev_meta_hash = genesis_meta.hash();
    
    for round in 1..=3 {
        println!("🎯 دوره ماینینگ {}", round);
        println!("{}", "-".repeat(30));
        
        let (block_a, block_b, meta_block) = run_mining_round(prev_meta_hash);
        
        println!("  بلوک A: chain_id={}, nonce={}", 
            block_a.chain_id, 
            block_a.header.nonce
        );
        println!("  بلوک B: chain_id={}, nonce={}", 
            block_b.chain_id, 
            block_b.header.nonce
        );
        println!("  متابلوک: height={}, hash={:?}", 
            round, 
            meta_block.hash()
        );
        
        prev_meta_hash = meta_block.hash();
        println!();
    }
    
    println!("✅ تست با موفقیت کامل شد!");
}