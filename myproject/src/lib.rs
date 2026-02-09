//! Parallel Bitcoin - یک پیاده‌سازی آزمایشی از بیت‌کوین با معماری موازی

pub mod chain;
pub mod consensus;
pub mod miner;
pub mod metablock;

// Re-export مهم‌ترین typeها
pub use chain::{ParallelBlock, MetaBlock};
pub use chain::params::ChainParams;
pub use miner::{SimpleMiner, ParallelMiner};
pub use consensus::calculate_next_target;

/// بلوک جنسیس شبکه
pub fn genesis_block(chain_id: u8) -> ParallelBlock {
    let params = ChainParams::new_testnet();
    params.create_genesis_block(chain_id)
}

/// اجرای یک دوره کامل ماینینگ موازی
pub fn run_mining_round(prev_meta_hash: bitcoin::hash_types::BlockHash) 
    -> (ParallelBlock, ParallelBlock, MetaBlock) 
{
    let params = std::sync::Arc::new(ChainParams::new_testnet());
    let miner = ParallelMiner::new(params);
    
    let (block_a, block_b) = miner.mine_parallel_blocks(prev_meta_hash);
    let meta_block = miner.create_meta_block(&block_a, &block_b, prev_meta_hash);
    
    (block_a, block_b, meta_block)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_genesis_blocks() {
        let block_a = genesis_block(0);
        let block_b = genesis_block(1);
        
        assert_eq!(block_a.chain_id, 0);
        assert_eq!(block_b.chain_id, 1);
        assert_ne!(block_a.hash(), block_b.hash());
    }
    
    #[test]
    fn test_meta_block_creation() {
        let genesis_hash = bitcoin::hash_types::BlockHash::all_zeros();
        let block_a = genesis_block(0);
        let block_b = genesis_block(1);
        
        let params = std::sync::Arc::new(ChainParams::new_testnet());
        let miner = ParallelMiner::new(params);
        
        let meta = miner.create_meta_block(&block_a, &block_b, genesis_hash);
        
        assert_eq!(meta.block_a_hash, block_a.hash());
        assert_eq!(meta.block_b_hash, block_b.hash());
    }
}