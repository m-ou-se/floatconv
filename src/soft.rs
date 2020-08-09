pub fn u32_to_f64(x: u32) -> f64 {
    if x == 0 { return 0.0; }
    let n = x.leading_zeros();
    let m = (x as u64) << (21 + n); // Significant bits, with bit 53 still in tact.
    let e = 1053 - n as u64; // Exponent plus 1023, minus one.
    f64::from_bits((e << 52) + m) // Bit 53 of m will overflow into e.
}

pub fn u64_to_f64_round(x: u64) -> f64 {
    if x == 0 { return 0.0; }
    let n = x.leading_zeros();
    let a = (x << n >> 11) as u64; // Significant bits, with bit 53 still in tact.
    let b = (x << n << 53) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 63 & !a) >> 63); // Add one when we need to round up. Break ties to even.
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    f64::from_bits((e << 52) + m) // + not |, so the mantissa can overflow into the exponent.
}

pub fn u64_to_f64_truncate(x: u64) -> f64 {
    if x == 0 { return 0.0; }
    let n = x.leading_zeros();
    let m = (x << n >> 11) as u64; // Significant bits, with bit 53 still in tact.
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    f64::from_bits((e << 52) + m) // + not |, so the mantissa can overflow into the exponent.
}

pub fn u128_to_f64_round(x: u128) -> f64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 63 & !a) >> 63); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
    f64::from_bits((e << 52) + m) // + not |, so the mantissa can overflow into the exponent.
}

pub fn u128_to_f64_truncate(x: u128) -> f64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let m = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let e = if x == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
    f64::from_bits((e << 52) + m) // + not |, so the mantissa can overflow into the exponent.
}

impl_signed!(i32_to_f64 i32 32 u32_to_f64);
impl_signed!(i64_to_f64_round i64 64 u64_to_f64_round);
impl_signed!(i64_to_f64_truncate i64 64 u64_to_f64_truncate);
impl_signed!(i128_to_f64_round i128 128 u128_to_f64_round);
impl_signed!(i128_to_f64_truncate i128 128 u128_to_f64_truncate);
