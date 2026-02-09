use bitcoin::hash_types::BlockHash;
use bitcoin::util::hash::Sha256dHash;

pub fn calculate_next_target(
    prev_bits: u32,
    actual_time: u64,
    expected_time: u64,
) -> u32 {
    // ساده‌سازی سختی – در آینده می‌تونی الگوریتم واقعی بیت‌کوین رو پیاده کنی
    let mut target = prev_bits;
    
    // اگر زمان واقعی خیلی بیشتر یا کمتر بود، سختی رو تنظیم کن
    let adjustment = actual_time as i64 - expected_time as i64;
    if adjustment > 0 {
        target = target.saturating_sub(adjustment as u32 / 100);
    } else if adjustment < 0 {
        target = target.saturating_add((-adjustment) as u32 / 100);
    }

    target.max(1) // حداقل سختی ۱
}

// فعلاً ساده – بعداً می‌تونی MetaBlock validation اضافه کنی
pub fn validate_meta_block(meta: &super::chain::MetaBlock) -> bool {
    // چک ساده: آیا حداقل یک بلاک موازی وجود داره؟
    !meta.block_hashes.is_empty()
}