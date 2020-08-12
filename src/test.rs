use crate::*;

#[test]
fn test_all_from_u8() {
    for i in 0..=u8::MAX {
        let a = f32::from_bits(soft::u8_to_f32(i));
        let b = i as f32;
        assert_eq!(a, b, "{} -> f32", i);
        let a = f64::from_bits(soft::u8_to_f64(i));
        let b = i as f64;
        assert_eq!(a, b, "{} -> f64", i);
    }
}

#[test]
fn test_all_from_u16() {
    for i in 0..=u16::MAX {
        let a = f32::from_bits(soft::u16_to_f32(i));
        let b = i as f32;
        assert_eq!(a, b, "{} -> f32", i);
        let a = f64::from_bits(soft::u16_to_f64(i));
        let b = i as f64;
        assert_eq!(a, b, "{} -> f64", i);
    }
}

#[test]
#[ignore]
fn test_all_from_u32() {
    for i in 0..=u32::MAX {
        let a = f32::from_bits(soft::u32_to_f32_round(i));
        let b = i as f32;
        assert_eq!(a, b, "{} -> f32", i);
        let a = f64::from_bits(soft::u32_to_f64(i));
        let b = i as f64;
        assert_eq!(a, b, "{} -> f64", i);
    }
}

#[test]
#[ignore]
fn test_all_from_f32() {
    for i in 0..=u32::MAX {
        let f = f32::from_bits(i);
        let a = soft::f32_to_u8(i);
        let b = f as u8;
        assert_eq!(a, b, "{:?} -> u8", f);
        let a = soft::f32_to_u16(i);
        let b = f as u16;
        assert_eq!(a, b, "{:?} -> u16", f);
        let a = soft::f32_to_u32(i);
        let b = f as u32;
        assert_eq!(a, b, "{:?} -> u32", f);
        let a = soft::f32_to_u64(i);
        let b = f as u64;
        assert_eq!(a, b, "{:?} -> u64", f);
        let a = soft::f32_to_u128(i);
        let b = f as u128;
        assert_eq!(a, b, "{:?} -> u128", f);
    }
}

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
        // f32:
        0b100000000000000000000000000000, // Exact match, no rounding
        0b100000000000000000000000100010, // Round to closest (up)
        0b100000000000000000000000010010, // Round to closest (down)
        0b100000000000000000000001100, // Tie, round to even (up)
        0b100000000000000000000000100, // Tie, round to even (down)
        1u32 << 25,
        1u32 << 24,
        1u32 << 23,
        1u32 << 22,
        (1u32 << 25) - 1,
        (1u32 << 24) - 1,
        (1u32 << 23) - 1,
        (1u32 << 22) - 1,
        (1u32 << 25) + 1,
        (1u32 << 24) + 1,
        (1u32 << 23) + 1,
        (1u32 << 22) + 1,
    ][..]
    {
        assert_eq!(soft::u32_to_f64(i), (i as f64).to_bits());
        assert_eq!(fast::u32_to_f64(i), i as f64);
        assert_eq!(soft::u32_to_f32_round(i), (i as f32).to_bits());
        assert_eq!(fast::u32_to_f32_round(i), i as f32);
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
        1u64 << 63,
        // f32:
        0b10000000000000000000000000000000000000000000000, // Exact match, no rounding
        0b10000000000000000000000010001000000000000000000, // Round to closest (up)
        0b10000000000000000000000001001000000000000000000, // Round to closest (down)
        0b10000000000000000000000110000000000000000000, // Tie, round to even (up)
        0b10000000000000000000000010000000000000000000, // Tie, round to even (down)
        1u64 << 25,
        1u64 << 24,
        1u64 << 23,
        1u64 << 22,
        (1u64 << 25) - 1,
        (1u64 << 24) - 1,
        (1u64 << 23) - 1,
        (1u64 << 22) - 1,
        (1u64 << 25) + 1,
        (1u64 << 24) + 1,
        (1u64 << 23) + 1,
        (1u64 << 22) + 1,
        // f64:
        0b10000000000000000000000000000000000000000000000000000000000, // Exact match, no rounding
        0b10000000000000000000000000000000000000000000000000000100010, // Round to closest (up)
        0b10000000000000000000000000000000000000000000000000000010010, // Round to closest (down)
        0b10000000000000000000000000000000000000000000000000001100, // Tie, round to even (up)
        0b10000000000000000000000000000000000000000000000000000100, // Tie, round to even (down)
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
        0b1000000000000000000000011111111111111111111111111111111111111111,
        0b1111111111111111111111110111111111111111111111111111111111111111,
    ][..]
    {
        assert_eq!(soft::u64_to_f32_round(i), (i as f32).to_bits());
        assert_eq!(fast::u64_to_f32_round(i), i as f32);
        let f = i as f64;
        let t = if f as u64 > i || i == u64::max_value() {
            f64::from_bits(f.to_bits() - 1)
        } else {
            f
        };
        assert_eq!(soft::u64_to_f64_round(i), f.to_bits());
        assert_eq!(soft::u64_to_f64_truncate(i), t.to_bits());
        assert_eq!(fast::u64_to_f64_round(i), f);
        assert_eq!(fast::u64_to_f64_truncate(i), t);
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
        // Mantissa of 2*52 bits, with last 32 bits set.
        0b100000000000000000000000000000000000000000000000000000000000000000000000011111111111111111111111111111111,
        // Mantissa of 2*52 bits, with bit 23 set.
        0b100000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000,
        // Mantissa of 2*52 bits, with last 23 bits set.
        0b100000000000000000000000000000000000000000000000000000000000000000000000000000000011111111111111111111111,
        // Mantissa of 128-32 bits, with last 24 bits set.
        0b1000000000000000000000000000000000000000000000000000000000000000000000000111111111111111111111111,
        1u128 << 127,
        2u128 << 126,
        3u128 << 126,
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
        u128::max_value() - (u128::max_value() >> 24),
        u128::max_value() - (u128::max_value() >> 23),
        u128::max_value() - (u128::max_value() >> 22),
    ][..]
    {
        assert_eq!(soft::u128_to_f32_round(i), (i as f32).to_bits());
        assert_eq!(fast::u128_to_f32_round(i), i as f32);
        let f = i as f64;
        let t = if f as u128 > i || i == u128::max_value() {
            f64::from_bits(f.to_bits() - 1)
        } else {
            f
        };
        assert_eq!(soft::u128_to_f64_round(i), f.to_bits());
        assert_eq!(soft::u128_to_f64_truncate(i), t.to_bits());
        assert_eq!(fast::u128_to_f64_round(i), f);
        assert_eq!(fast::u128_to_f64_truncate(i), t);
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
        assert_eq!(soft::i32_to_f64(i), (i as f64).to_bits());
        assert_eq!(fast::i32_to_f64(i), i as f64);
        assert_eq!(soft::i32_to_f32_round(i), (i as f32).to_bits());
        assert_eq!(fast::i32_to_f32_round(i), i as f32);
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
        assert_eq!(soft::i64_to_f32_round(i), (i as f32).to_bits());
        assert_eq!(fast::i64_to_f32_round(i), i as f32);
        assert_eq!(soft::i64_to_f64_round(i), (i as f64).to_bits());
        assert_eq!(fast::i64_to_f64_round(i), i as f64);
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
        assert_eq!(soft::i128_to_f32_round(i), (i as f32).to_bits());
        assert_eq!(fast::i128_to_f32_round(i), i as f32);
        assert_eq!(soft::i128_to_f64_round(i), (i as f64).to_bits());
        assert_eq!(fast::i128_to_f64_round(i), i as f64);
    }
}
