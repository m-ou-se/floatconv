//! Software implementations of all conversion functions that don't use any
//! floating point instructions.
//!
//! Available on all platforms.
//!
//! To avoid using any floating point instructions or registers, all functions
//! in this module take or return the bits of the floating point value as `u32`
//! or `u64` instead of `f32` or `f64`.

#[cfg_attr(not(noinline), inline)]
pub fn u8_to_f32(x: u8) -> u32 {
    u16_to_f32(x.into())
}

#[cfg_attr(not(noinline), inline)]
pub fn u16_to_f32(x: u16) -> u32 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let m = (x as u32) << (8 + n); // Significant bits, with bit 53 still in tact.
    let e = 141 - n; // Exponent plus 127, minus one.
    (e << 23) + m // Bit 24 of m will overflow into e.
}

#[cfg_attr(not(noinline), inline)]
pub fn u32_to_f32(x: u32) -> u32 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let a = x << n >> 8; // Significant bits, with bit 24 still in tact.
    let b = x << n << 24; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 31 & !a)) >> 31); // Add one when we need to round up. Break ties to even.
    let e = 157 - n as u32; // Exponent plus 127, minus one.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn u64_to_f32(x: u64) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 40) as u32; // Significant bits, with bit 24 still in tact.
    let b = (y >> 8 | y & 0xFFFF) as u32; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 31 & !a)) >> 31); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 189 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn u128_to_f32(x: u128) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 104) as u32; // Significant bits, with bit 24 still in tact.
    let b = (y >> 72) as u32 | (y << 32 >> 32 != 0) as u32; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 31 & !a)) >> 31); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 253 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn u8_to_f64(x: u8) -> u64 {
    u32_to_f64(x.into())
}

#[cfg_attr(not(noinline), inline)]
pub fn u16_to_f64(x: u16) -> u64 {
    u32_to_f64(x.into())
}

#[cfg_attr(not(noinline), inline)]
pub fn u32_to_f64(x: u32) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let m = (x as u64) << (21 + n); // Significant bits, with bit 53 still in tact.
    let e = 1053 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // Bit 53 of m will overflow into e.
}

#[cfg_attr(not(noinline), inline)]
pub fn u64_to_f64(x: u64) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let a = (x << n >> 11) as u64; // Significant bits, with bit 53 still in tact.
    let b = (x << n << 53) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 63 & !a)) >> 63); // Add one when we need to round up. Break ties to even.
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn u128_to_f64(x: u128) -> u64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 63 & !a)) >> 63); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn i8_to_f32(i: i8) -> u32 {
    let sign_bit = ((i >> 7) as u32) << 31;
    u8_to_f32(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i16_to_f32(i: i16) -> u32 {
    let sign_bit = ((i >> 15) as u32) << 31;
    u16_to_f32(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i32_to_f32(i: i32) -> u32 {
    let sign_bit = ((i >> 31) as u32) << 31;
    u32_to_f32(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i64_to_f32(i: i64) -> u32 {
    let sign_bit = ((i >> 63) as u32) << 31;
    u64_to_f32(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i128_to_f32(i: i128) -> u32 {
    let sign_bit = ((i >> 127) as u32) << 31;
    u128_to_f32(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i8_to_f64(i: i8) -> u64 {
    let sign_bit = ((i >> 7) as u64) << 63;
    u8_to_f64(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i16_to_f64(i: i16) -> u64 {
    let sign_bit = ((i >> 15) as u64) << 63;
    u16_to_f64(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i32_to_f64(i: i32) -> u64 {
    let sign_bit = ((i >> 31) as u64) << 63;
    u32_to_f64(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i64_to_f64(i: i64) -> u64 {
    let sign_bit = ((i >> 63) as u64) << 63;
    u64_to_f64(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn i128_to_f64(i: i128) -> u64 {
    let sign_bit = ((i >> 127) as u64) << 63;
    u128_to_f64(i.unsigned_abs()) | sign_bit
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_u8(f: u32) -> u8 {
    if f < 127 << 23 { // >= 0, < 1
        0
    } else if f < 135 << 23 { // >= 1, < max
        let m = 1 << 7 | (f >> 16) as u8; // Mantissa and the implicit 1-bit.
        let s = 134 - (f >> 23); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 255 << 23 { // >= max (incl. inf)
        u8::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_u16(f: u32) -> u16 {
    if f < 127 << 23 { // >= 0, < 1
        0
    } else if f < 143 << 23 { // >= 1, < max
        let m = 1 << 15 | (f >> 8) as u16; // Mantissa and the implicit 1-bit.
        let s = 142 - (f >> 23); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 255 << 23 { // >= max (incl. inf)
        u16::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_u32(f: u32) -> u32 {
    if f < 127 << 23 { // >= 0, < 1
        0
    } else if f < 159 << 23 { // >= 1, < max
        let m = 1 << 31 | f << 8; // Mantissa and the implicit 1-bit.
        let s = 158 - (f >> 23); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 255 << 23 { // >= max (incl. inf)
        u32::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_u64(f: u32) -> u64 {
    if f < 127 << 23 { // >= 0, < 1
        0
    } else if f < 191 << 23 { // >= 1, < max
        let m = 1 << 63 | (f as u64) << 40; // Mantissa and the implicit 1-bit.
        let s = 190 - (f >> 23); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 255 << 23 { // >= max (incl. inf)
        u64::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_u128(f: u32) -> u128 {
    if f < 127 << 23 { // >= 0, < 1
        0
    } else if f < 255 << 23 { // >= 1, < inf
        let m = 1 << 127 | (f as u128) << 104; // Mantissa and the implicit 1-bit.
        let s = 254 - (f >> 23); // Shift based on the exponent and bias.
        m >> s
    } else if f == 255 << 23 { // == inf
        u128::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_u8(f: u64) -> u8 {
    if f < 1023 << 52 { // >= 0, < 1
        0
    } else if f < 1031 << 52 { // >= 1, < max
        let m = 1 << 7 | (f >> 45) as u8; // Mantissa and the implicit 1-bit.
        let s = 1030 - (f >> 52); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 2047 << 52 { // >= max (incl. inf)
        u8::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_u16(f: u64) -> u16 {
    if f < 1023 << 52 { // >= 0, < 1
        0
    } else if f < 1039 << 52 { // >= 1, < max
        let m = 1 << 15 | (f >> 37) as u16; // Mantissa and the implicit 1-bit.
        let s = 1038 - (f >> 52); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 2047 << 52 { // >= max (incl. inf)
        u16::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_u32(f: u64) -> u32 {
    if f < 1023 << 52 { // >= 0, < 1
        0
    } else if f < 1055 << 52 { // >= 1, < max
        let m = 1 << 31 | (f >> 21) as u32; // Mantissa and the implicit 1-bit.
        let s = 1054 - (f >> 52); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 2047 << 52 { // >= max (incl. inf)
        u32::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_u64(f: u64) -> u64 {
    if f < 1023 << 52 { // >= 0, < 1
        0
    } else if f < 1087 << 52 { // >= 1, < max
        let m = 1 << 63 | f << 11; // Mantissa and the implicit 1-bit.
        let s = 1086 - (f >> 52); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 2047 << 52 { // >= max (incl. inf)
        u64::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_u128(f: u64) -> u128 {
    if f < 1023 << 52 { // >= 0, < 1
        0
    } else if f < 1151 << 52 { // >= 1, < max
        let m = 1 << 127 | (f as u128) << 75; // Mantissa and the implicit 1-bit.
        let s = 1150 - (f >> 52); // Shift based on the exponent and bias.
        m >> s
    } else if f <= 2047 << 52 { // >= max (incl. inf)
        u128::MAX
    } else { // Negative or NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_i8(f: u32) -> i8 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 127 << 23 { // >= 0, < 1
        0
    } else if a < 134 << 23 { // >= 1, < max
        let m = 1 << 7 | (a >> 16) as u8; // Mantissa and the implicit 1-bit.
        let s = 134 - (a >> 23); // Shift based on the exponent and bias.
        let u = (m >> s) as i8; // Unsigned result.
        if (f as i32) < 0 { -u } else { u }
    } else if a <= 255 << 23 { // >= max (incl. inf)
        if (f as i32) < 0 { i8::MIN } else { i8::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_i16(f: u32) -> i16 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 127 << 23 { // >= 0, < 1
        0
    } else if a < 142 << 23 { // >= 1, < max
        let m = 1 << 15 | (a >> 8) as u16; // Mantissa and the implicit 1-bit.
        let s = 142 - (a >> 23); // Shift based on the exponent and bias.
        let u = (m >> s) as i16; // Unsigned result.
        if (f as i32) < 0 { -u } else { u }
    } else if a <= 255 << 23 { // >= max (incl. inf)
        if (f as i32) < 0 { i16::MIN } else { i16::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_i32(f: u32) -> i32 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 127 << 23 { // >= 0, < 1
        0
    } else if a < 158 << 23 { // >= 1, < max
        let m = 1 << 31 | a << 8; // Mantissa and the implicit 1-bit.
        let s = 158 - (a >> 23); // Shift based on the exponent and bias.
        let u = (m >> s) as i32; // Unsigned result.
        if (f as i32) < 0 { -u } else { u }
    } else if a <= 255 << 23 { // >= max (incl. inf)
        if (f as i32) < 0 { i32::MIN } else { i32::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_i64(f: u32) -> i64 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 127 << 23 { // >= 0, < 1
        0
    } else if a < 190 << 23 { // >= 1, < max
        let m = 1 << 63 | (a as u64) << 40; // Mantissa and the implicit 1-bit.
        let s = 190 - (a >> 23); // Shift based on the exponent and bias.
        let u = (m >> s) as i64; // Unsigned result.
        if (f as i32) < 0 { -u } else { u }
    } else if a <= 255 << 23 { // >= max (incl. inf)
        if (f as i32) < 0 { i64::MIN } else { i64::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f32_to_i128(f: u32) -> i128 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 127 << 23 { // >= 0, < 1
        0
    } else if a < 254 << 23 { // >= 1, < max
        let m = 1 << 127 | (a as u128) << 104; // Mantissa and the implicit 1-bit.
        let s = 254 - (a >> 23); // Shift based on the exponent and bias.
        let u = (m >> s) as i128; // Unsigned result.
        if (f as i32) < 0 { -u } else { u }
    } else if a <= 255 << 23 { // >= max (incl. inf)
        if (f as i32) < 0 { i128::MIN } else { i128::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_i8(f: u64) -> i8 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 1023 << 52 { // >= 0, < 1
        0
    } else if a < 1030 << 52 { // >= 1, < max
        let m = 1 << 7 | (a >> 45) as u8; // Mantissa and the implicit 1-bit.
        let s = 1030 - (a >> 52); // Shift based on the exponent and bias.
        let u = (m >> s) as i8; // Unsigned result.
        if (f as i64) < 0 { -u } else { u }
    } else if a <= 2047 << 52 { // >= max (incl. inf)
        if (f as i64) < 0 { i8::MIN } else { i8::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_i16(f: u64) -> i16 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 1023 << 52 { // >= 0, < 1
        0
    } else if a < 1038 << 52 { // >= 1, < max
        let m = 1 << 15 | (a >> 37) as u16; // Mantissa and the implicit 1-bit.
        let s = 1038 - (a >> 52); // Shift based on the exponent and bias.
        let u = (m >> s) as i16; // Unsigned result.
        if (f as i64) < 0 { -u } else { u }
    } else if a <= 2047 << 52 { // >= max (incl. inf)
        if (f as i64) < 0 { i16::MIN } else { i16::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_i32(f: u64) -> i32 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 1023 << 52 { // >= 0, < 1
        0
    } else if a < 1054 << 52 { // >= 1, < max
        let m = 1 << 31 | (a >> 21) as u32; // Mantissa and the implicit 1-bit.
        let s = 1054 - (a >> 52); // Shift based on the exponent and bias.
        let u = (m >> s) as i32; // Unsigned result.
        if (f as i64) < 0 { -u } else { u }
    } else if a <= 2047 << 52 { // >= max (incl. inf)
        if (f as i64) < 0 { i32::MIN } else { i32::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_i64(f: u64) -> i64 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 1023 << 52 { // >= 0, < 1
        0
    } else if a < 1086 << 52 { // >= 1, < max
        let m = 1 << 63 | a << 11; // Mantissa and the implicit 1-bit.
        let s = 1086 - (a >> 52); // Shift based on the exponent and bias.
        let u = (m >> s) as i64; // Unsigned result.
        if (f as i64) < 0 { -u } else { u }
    } else if a <= 2047 << 52 { // >= max (incl. inf)
        if (f as i64) < 0 { i64::MIN } else { i64::MAX }
    } else { // NaN
        0
    }
}

#[cfg_attr(not(noinline), inline)]
pub fn f64_to_i128(f: u64) -> i128 {
    let a = f & !0 >> 1; // Remove sign bit.
    if a < 1023 << 52 { // >= 0, < 1
        0
    } else if a < 1150 << 52 { // >= 1, < max
        let m = 1 << 127 | (a as u128) << 75; // Mantissa and the implicit 1-bit.
        let s = 1150 - (a >> 52); // Shift based on the exponent and bias.
        let u = (m >> s) as i128; // Unsigned result.
        if (f as i64) < 0 { -u } else { u }
    } else if a <= 2047 << 52 { // >= max (incl. inf)
        if (f as i64) < 0 { i128::MIN } else { i128::MAX }
    } else { // NaN
        0
    }
}
