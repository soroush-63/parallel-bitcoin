use bitcoin::blockdata::block::BlockHeader;
use bitcoin::consensus::encode;
use std::time::{SystemTime, UNIX_EPOCH};

/// محاسبه سختی جدید (ساده‌شده)
pub fn calculate_next_target(
    previous_headers: &[BlockHeader],
    target_spacing: u64,
    adjustment_interval: u32,
    pow_limit: [u8; 32],
) -> Result<u32, String> {
    if previous_headers.len() < adjustment_interval as usize {
        // هنوز به اندازه کافی بلوک نداریم
        return Ok(previous_headers.last().unwrap().bits);
    }
    
    // محاسبه زمان واقعی تولید بلوک‌ها
    let first_time = previous_headers[0].time;
    let last_time = previous_headers[previous_headers.len() - 1].time;
    let actual_timespan = (last_time - first_time) as u64;
    
    // زمان ایده‌آل
    let target_timespan = target_spacing * adjustment_interval as u64;
    
    // محدود کردن تغییرات
    let mut adjusted_timespan = actual_timespan;
    if adjusted_timespan < target_timespan / 4 {
        adjusted_timespan = target_timespan / 4;
    }
    if adjusted_timespan > target_timespan * 4 {
        adjusted_timespan = target_timespan * 4;
    }
    
    // محاسبه target جدید (ساده‌شده)
    let old_target = decode_bits(previous_headers.last().unwrap().bits);
    let new_target = old_target * adjusted_timespan / target_timespan;
    
    // محدود به pow_limit
    let limit = u256_from_bytes(&pow_limit);
    let final_target = if new_target > limit { limit } else { new_target };
    
    Ok(encode_bits(final_target))
}

/// تبدیل bits به عدد 256 بیتی (ساده‌شده)
fn decode_bits(bits: u32) -> u128 {
    // پیاده‌سازی ساده برای تست
    let exponent = (bits >> 24) as u32;
    let coefficient = bits & 0x007fffff;
    coefficient as u128 * 2u128.pow(8 * (exponent - 3))
}

/// تبدیل عدد به bits (ساده‌شده)
fn encode_bits(mut target: u128) -> u32 {
    // پیاده‌سازی ساده برای تست
    let mut exponent = 32;
    while target > 0x007fffff {
        target >>= 8;
        exponent += 1;
    }
    ((exponent as u32) << 24) | (target as u32 & 0x007fffff)
}

fn u256_from_bytes(bytes: &[u8; 32]) -> u128 {
    // ساده‌سازی: فقط 16 بایت اول
    let mut result = 0u128;
    for i in 0..16 {
        result |= (bytes[15 - i] as u128) << (8 * i);
    }
    result
}