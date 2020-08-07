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

macro_rules! impl_signed {
    (
        $name:tt
        $from:tt
        $bits:tt
        $unsigned:tt
    ) => (
        pub fn $name(x: $from) -> f64 {
            let s = ((x >> $bits - 1) as u64) << 63;
            f64::from_bits($unsigned(x.wrapping_abs() as _).to_bits() | s)
        }
    );
}

impl_signed!(i32_to_f64 i32 32 u32_to_f64);
impl_signed!(i64_to_f64_round i64 64 u64_to_f64_round);
impl_signed!(i64_to_f64_truncate i64 64 u64_to_f64_truncate);
impl_signed!(i128_to_f64_round i128 128 u128_to_f64_round);
impl_signed!(i128_to_f64_truncate i128 128 u128_to_f64_truncate);

#[test]
fn test_u32() {
    for &i in &[
        0,
        1,
        2,
        3,
        1234,
        u32::max_value(),
        u32::max_value() - 1,
        u32::max_value() / 2,
        u32::min_value(),
        u32::min_value() + 1,
        u32::min_value() / 2,
        123123123,
        321312312,
    ][..]
    {
        assert_eq!(u32_to_f64(i), i as f64);
    }
}

#[test]
fn test_u64() {
    for &i in &[
        0,
        1,
        2,
        3,
        1234,
        u64::max_value(), // Overflows the mantissa, should increment the exponent (which will be odd).
        u64::max_value() / 2, // Overflows the mantissa, should increment the exponent (which will be even).
        0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        0b10000000000000000000000000000000000000000000000000001100, // Tie, round to even (up)
        0b10000000000000000000000000000000000000000000000000000100, // Tie, round to even (down)
        1u64 << 63,
        1u64 << 54,
        1u64 << 53,
        1u64 << 52,
        1u64 << 51,
        (1u64 << 54) - 1,
        (1u64 << 53) - 1,
        (1u64 << 52) - 1,
        (1u64 << 51) - 1,
        (1u64 << 54) + 1,
        (1u64 << 53) + 1,
        (1u64 << 52) + 1,
        (1u64 << 51) + 1,
    ][..]
    {
        let f = i as f64;
        let t = if f as u64 > i || i == u64::max_value() {
            f64::from_bits(f.to_bits() - 1)
        } else {
            f
        };
        assert_eq!(u64_to_f64_round(i), f);
        assert_eq!(u64_to_f64_truncate(i), t);
    }
}

#[test]
fn test_u128() {
    for &i in &[
        0,
        1,
        2,
        3,
        1234,
        u128::max_value(), // Overflows the mantissa, should increment the exponent (which will be odd).
        u128::max_value() / 2, // Overflows the mantissa, should increment the exponent (which will be even).
        0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        0b10000000000000000000000000000000000000000000000000001100, // Tie, round to even (up)
        0b10000000000000000000000000000000000000000000000000000100, // Tie, round to even (down)
        // Round to closest (up), with tie-breaking bit further than 64 bits away.
        0b10000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000001,
        // Round to closest (down), with 1-bit in 63rd position (which should be insignificant).
        0b10000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000,
        // Round to closest (down), with 1-bits in all insignificant positions.
        0b10000000000000000000000000000000000000000000000000000011111111111111111111111111111111111111111111111111111111111111111111111111,
        1u128 << 127,
        1u128 << 64,
        1u128 << 63,
        1u128 << 54,
        1u128 << 53,
        1u128 << 52,
        1u128 << 51,
        (1u128 << 54) - 1,
        (1u128 << 53) - 1,
        (1u128 << 52) - 1,
        (1u128 << 51) - 1,
        (1u128 << 54) + 1,
        (1u128 << 53) + 1,
        (1u128 << 52) + 1,
        (1u128 << 51) + 1,
        u128::from(u64::max_value()),
        u128::from(u64::max_value()) << 64,
        u128::from(u64::max_value()) << 63,
        u128::from(u64::max_value()) << 53,
        u128::from(u64::max_value()) << 52,
        u128::from(u64::max_value()) << 51,
        u128::from(u64::max_value() >> 13) << 64,
        u128::from(u64::max_value() >> 13) << 63,
        u128::from(u64::max_value() >> 13) << 53,
        u128::from(u64::max_value() >> 13) << 52,
        u128::from(u64::max_value() >> 13) << 51,
        u128::from(u64::max_value() >> 12) << 64,
        u128::from(u64::max_value() >> 12) << 63,
        u128::from(u64::max_value() >> 12) << 53,
        u128::from(u64::max_value() >> 12) << 52,
        u128::from(u64::max_value() >> 12) << 51,
        u128::from(u64::max_value() >> 11) << 64,
        u128::from(u64::max_value() >> 11) << 63,
        u128::from(u64::max_value() >> 11) << 53,
        u128::from(u64::max_value() >> 11) << 52,
        u128::from(u64::max_value() >> 11) << 51,
    ][..]
    {
        let f = i as f64;
        let t = if f as u128 > i || i == u128::max_value() {
            f64::from_bits(f.to_bits() - 1)
        } else {
            f
        };
        assert_eq!(u128_to_f64_round(i), f);
        assert_eq!(u128_to_f64_truncate(i), t);
    }
}

#[test]
fn test_i32() {
    for &i in &[
        0,
        1,
        -1,
        2,
        -2,
        3,
        -3,
        1234,
        i32::max_value(),
        i32::max_value() / 2,
        123123123,
        321312312,
    ][..]
    {
        assert_eq!(i32_to_f64(i), i as f64);
    }
}

#[test]
fn test_i64() {
    for &i in &[
        0,
        1,
        2,
        3,
        1234,
        -0,
        -1,
        -2,
        -3,
        -1234,
        i64::max_value(),
        i64::max_value() - 1,
        i64::max_value() / 2,
        i64::min_value(),
        i64::min_value() + 1,
        i64::min_value() / 2,
        0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        0b10000000000000000000000000000000000000000000000000001100,    // Tie, round to even (up)
        0b10000000000000000000000000000000000000000000000000000100,    // Tie, round to even (down)
        -0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        -0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        -0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        -0b10000000000000000000000000000000000000000000000000001100,   // Tie, round to even (up)
        -0b10000000000000000000000000000000000000000000000000000100,   // Tie, round to even (down)
        1i64 << 63,
        1i64 << 54,
        1i64 << 53,
        1i64 << 52,
        1i64 << 51,
        (1i64 << 54) - 1,
        (1i64 << 53) - 1,
        (1i64 << 52) - 1,
        (1i64 << 51) - 1,
        (1i64 << 54) + 1,
        (1i64 << 53) + 1,
        (1i64 << 52) + 1,
        (1i64 << 51) + 1,
        -(1i64 << 54),
        -(1i64 << 53),
        -(1i64 << 52),
        -(1i64 << 51),
        -(1i64 << 54) - 1,
        -(1i64 << 53) - 1,
        -(1i64 << 52) - 1,
        -(1i64 << 51) - 1,
        -(1i64 << 54) + 1,
        -(1i64 << 53) + 1,
        -(1i64 << 52) + 1,
        -(1i64 << 51) + 1,
    ][..]
    {
        assert_eq!(i64_to_f64_round(i), i as f64);
    }
}

#[test]
fn test_i128() {
    for &i in &[
        0,
        1,
        2,
        3,
        1234,
        -0,
        -1,
        -2,
        -3,
        -1234,
        i128::max_value(),
        i128::max_value() - 1,
        i128::max_value() / 2,
        i128::min_value(),
        i128::min_value() + 1,
        i128::min_value() / 2,
        0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        0b10000000000000000000000000000000000000000000000000001100,    // Tie, round to even (up)
        0b10000000000000000000000000000000000000000000000000000100,    // Tie, round to even (down)
        -0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        -0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        -0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        -0b10000000000000000000000000000000000000000000000000001100,   // Tie, round to even (up)
        -0b10000000000000000000000000000000000000000000000000000100,   // Tie, round to even (down)
        1i128 << 127,
        1i128 << 64,
        1i128 << 63,
        1i128 << 54,
        1i128 << 53,
        1i128 << 52,
        1i128 << 51,
        (1i128 << 54) - 1,
        (1i128 << 53) - 1,
        (1i128 << 52) - 1,
        (1i128 << 51) - 1,
        (1i128 << 54) + 1,
        (1i128 << 53) + 1,
        (1i128 << 52) + 1,
        (1i128 << 51) + 1,
        -(1i128 << 64),
        -(1i128 << 63),
        -(1i128 << 54),
        -(1i128 << 53),
        -(1i128 << 52),
        -(1i128 << 51),
        -(1i128 << 54) - 1,
        -(1i128 << 53) - 1,
        -(1i128 << 52) - 1,
        -(1i128 << 51) - 1,
        -(1i128 << 54) + 1,
        -(1i128 << 53) + 1,
        -(1i128 << 52) + 1,
        -(1i128 << 51) + 1,
    ][..]
    {
        assert_eq!(i128_to_f64_round(i), i as f64);
    }
}
