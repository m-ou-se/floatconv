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
    let m = a + (b - (b >> 31 & !a) >> 31); // Add one when we need to round up. Break ties to even.
    let e = if x == 0 { 0 } else { 189 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn u128_to_f32(x: u128) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 104) as u32; // Significant bits, with bit 24 still in tact.
    let b = (y >> 72) as u32 | (y << 32 >> 32 != 0) as u32; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 31 & !a) >> 31); // Add one when we need to round up. Break ties to even.
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
    let m = a + (b - (b >> 63 & !a) >> 63); // Add one when we need to round up. Break ties to even.
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

#[cfg_attr(not(noinline), inline)]
pub fn u128_to_f64(x: u128) -> u64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + (b - (b >> 63 & !a) >> 63); // Add one when we need to round up. Break ties to even.
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

macro_rules! impl_to_int {
    ($f:tt $s:tt $e:tt $n:tt $t:tt $u:tt $signed:tt) => {
        #[cfg_attr(not(noinline), inline)]
        pub fn $n(mut x: $f) -> $t {
            const BITS: u32 = $t::MAX.count_ones();
            const FBITS: u32 = $f::MAX.count_ones();
            let s = $signed && x >> FBITS - 1 > 0;
            if s { x &= !0 >> 1; } // Remove sign
            if x > $e * 2 + 1 << $s { // Negative or ±NaN
                0
            } else if x >= $e + (BITS as $f) << $s { // >= max (incl. inf)
                if s { $t::MIN } else { $t::MAX }
            } else if x >= $e << $s { // >= 1, < max
                const R: u32 = if BITS > $s { 0 } else { $s - BITS };
                const L: u32 = if BITS < $s { 0 } else { BITS - $s };
                let y = (((x >> R) as $u) << L >> 1 | 1 << BITS - 1) >> $e + BITS - 1 - (x >> $s) as u32;
                (if s { !y + 1 } else { y }) as $t
            } else { // >= 0, < 1
                0
            }
        }
    }
}

impl_to_int!(u32 23 127 f32_to_u8 u8 u8 false);
impl_to_int!(u32 23 127 f32_to_u16 u16 u16 false);
impl_to_int!(u32 23 127 f32_to_u32 u32 u32 false);
impl_to_int!(u32 23 127 f32_to_u64 u64 u64 false);
impl_to_int!(u32 23 127 f32_to_u128 u128 u128 false);

impl_to_int!(u32 23 127 f32_to_i8 i8 u8 true);
impl_to_int!(u32 23 127 f32_to_i16 i16 u16 true);
impl_to_int!(u32 23 127 f32_to_i32 i32 u32 true);
impl_to_int!(u32 23 127 f32_to_i64 i64 u64 true);
impl_to_int!(u32 23 127 f32_to_i128 i128 u128 true);

impl_to_int!(u64 52 1023 f64_to_u8 u8 u8 false);
impl_to_int!(u64 52 1023 f64_to_u16 u16 u16 false);
impl_to_int!(u64 52 1023 f64_to_u32 u32 u32 false);
impl_to_int!(u64 52 1023 f64_to_u64 u64 u64 false);
impl_to_int!(u64 52 1023 f64_to_u128 u128 u128 false);

impl_to_int!(u64 52 1023 f64_to_i8 i8 u8 true);
impl_to_int!(u64 52 1023 f64_to_i16 i16 u16 true);
impl_to_int!(u64 52 1023 f64_to_i32 i32 u32 true);
impl_to_int!(u64 52 1023 f64_to_i64 i64 u64 true);
impl_to_int!(u64 52 1023 f64_to_i128 i128 u128 true);
