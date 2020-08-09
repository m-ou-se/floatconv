//! Software implementations of all conversion functions that don't use any
//! floating point instructions.
//!
//! Available on all platforms.
//!
//! To avoid using any floating point instructions or registers, all functions
//! in this module return the bits of the floating point value as `u32` or
//! `u64` instead of `f32` or `f64`.

pub fn u32_to_f32_round(x: u32) -> u32 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let a = x << n >> 8; // Significant bits, with bit 24 still in tact.
    let b = x << n << 24; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 31 & !a) >> 31); // Add one when we need to round up. Break ties to even.
    let e = 157 - n as u32; // Exponent plus 127, minus one.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

pub fn u64_to_f32_round(x: u64) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 40) as u32; // Significant bits, with bit 24 still in tact.
    let b = (y >> 8 | y & 0xFFFF) as u32; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 31 & !a) >> 31); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 189 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

pub fn u128_to_f32_round(x: u128) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 104) as u32; // Significant bits, with bit 24 still in tact.
    let b = (y >> 72) as u32 | (y << 32 >> 32 != 0) as u32; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 31 & !a) >> 31); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 189 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

pub fn u32_to_f64(x: u32) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let m = (x as u64) << (21 + n); // Significant bits, with bit 53 still in tact.
    let e = 1053 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // Bit 53 of m will overflow into e.
}

pub fn u64_to_f64_round(x: u64) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let a = (x << n >> 11) as u64; // Significant bits, with bit 53 still in tact.
    let b = (x << n << 53) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 63 & !a) >> 63); // Add one when we need to round up. Break ties to even.
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

pub fn u64_to_f64_truncate(x: u64) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let m = (x << n >> 11) as u64; // Significant bits, with bit 53 still in tact.
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

pub fn u128_to_f64_round(x: u128) -> u64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 63 & !a) >> 63); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

pub fn u128_to_f64_truncate(x: u128) -> u64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let m = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let e = if x == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

macro_rules! impl_signed {
    ($name:tt $from:tt $bits:tt $unsigned:tt $return:tt) => {
        pub fn $name(x: $from) -> $return {
            let s = ((x >> $bits - 1) as $return) << core::mem::size_of::<$return>() * 8 - 1;
            $unsigned(x.wrapping_abs() as _) | s
        }
    };
}

impl_signed!(i32_to_f32_round i32 32 u32_to_f32_round u32);
impl_signed!(i64_to_f32_round i64 64 u64_to_f32_round u32);
impl_signed!(i128_to_f32_round i128 128 u128_to_f32_round u32);
impl_signed!(i32_to_f64 i32 32 u32_to_f64 u64);
impl_signed!(i64_to_f64_round i64 64 u64_to_f64_round u64);
impl_signed!(i64_to_f64_truncate i64 64 u64_to_f64_truncate u64);
impl_signed!(i128_to_f64_round i128 128 u128_to_f64_round u64);
impl_signed!(i128_to_f64_truncate i128 128 u128_to_f64_truncate u64);
