//! Software implementations of all conversion functions that don't use any
//! floating point instructions.
//!
//! Available on all platforms.
//!
//! To avoid using any floating point instructions or registers, all functions
//! in this module take or return the bits of the floating point value as `u32`
//! or `u64` instead of `f32` or `f64`.

#[inline]
pub fn u8_to_f32(x: u8) -> u32 {
    u16_to_f32(x.into())
}

#[inline]
pub fn u16_to_f32(x: u16) -> u32 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let m = (x as u32) << (8 + n); // Significant bits, with bit 53 still in tact.
    let e = 141 - n; // Exponent plus 127, minus one.
    (e << 23) + m // Bit 24 of m will overflow into e.
}

#[inline]
fn u32_to_f32(x: u32, round: bool) -> u32 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let mut m = x << n >> 8; // Significant bits, with bit 24 still in tact.
    if round {
        let b = x << n << 24; // Insignificant bits, only relevant for rounding.
        m += b - (b >> 31 & !m) >> 31; // Add one when we need to round up. Break ties to even.
    }
    let e = 157 - n as u32; // Exponent plus 127, minus one.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[inline]
fn u64_to_f32(x: u64, round: bool) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let mut m = (y >> 40) as u32; // Significant bits, with bit 24 still in tact.
    if round {
        let b = (y >> 8 | y & 0xFFFF) as u32; // Insignificant bits, only relevant for rounding.
        m += b - (b >> 31 & !m) >> 31; // Add one when we need to round up. Break ties to even.
    }
    let e = if x == 0 { 0 } else { 189 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[inline]
fn u128_to_f32(x: u128, round: bool) -> u32 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let mut m = (y >> 104) as u32; // Significant bits, with bit 24 still in tact.
    if round {
        let b = (y >> 72) as u32 | (y << 32 >> 32 != 0) as u32; // Insignificant bits, only relevant for rounding.
        m += b - (b >> 31 & !m) >> 31; // Add one when we need to round up. Break ties to even.
    }
    let e = if x == 0 { 0 } else { 253 - n }; // Exponent plus 127, minus one, except for zero.
    (e << 23) + m // + not |, so the mantissa can overflow into the exponent.
}

#[inline]
pub fn u8_to_f64(x: u8) -> u64 {
    u32_to_f64(x.into())
}

#[inline]
pub fn u16_to_f64(x: u16) -> u64 {
    u32_to_f64(x.into())
}

#[inline]
pub fn u32_to_f64(x: u32) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let m = (x as u64) << (21 + n); // Significant bits, with bit 53 still in tact.
    let e = 1053 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // Bit 53 of m will overflow into e.
}

#[inline]
fn u64_to_f64(x: u64, round: bool) -> u64 {
    if x == 0 { return 0; }
    let n = x.leading_zeros();
    let mut m = (x << n >> 11) as u64; // Significant bits, with bit 53 still in tact.
    if round {
        let b = (x << n << 53) as u64; // Insignificant bits, only relevant for rounding.
        m += b - (b >> 63 & !m) >> 63; // Add one when we need to round up. Break ties to even.
    }
    let e = 1085 - n as u64; // Exponent plus 1023, minus one.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

#[inline]
fn u128_to_f64(x: u128, round: bool) -> u64 {
    let n = x.leading_zeros();
    let y = x.wrapping_shl(n);
    let mut m = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    if round {
        let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
        m += b - (b >> 63 & !m) >> 63; // Add one when we need to round up. Break ties to even.
    }
    let e = if x == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

macro_rules! impl_signed {
    ($name:tt $from:tt $unsigned:tt $return:tt) => {
        #[inline]
        pub fn $name(x: $from) -> $return {
            let from_bits = core::mem::size_of::<$from>() * 8;
            let return_bits = core::mem::size_of::<$return>() * 8;
            let s = ((x >> from_bits - 1) as $return) << return_bits - 1;
            $unsigned(x.wrapping_abs() as _) | s
        }
    };
}

macro_rules! impl_round {
    ($name_round:tt $name_truncate:tt $from:tt $return:tt $name:tt) => {
        #[inline]
        pub fn $name_round(x: $from) -> $return {
            $name(x, true)
        }
        #[inline]
        pub fn $name_truncate(x: $from) -> $return {
            $name(x, false)
        }
    };
}

impl_round!(u32_to_f32_round u32_to_f32_truncate u32 u32 u32_to_f32);
impl_round!(u64_to_f32_round u64_to_f32_truncate u64 u32 u64_to_f32);
impl_round!(u128_to_f32_round u128_to_f32_truncate u128 u32 u128_to_f32);
impl_round!(u64_to_f64_round u64_to_f64_truncate u64 u64 u64_to_f64);
impl_round!(u128_to_f64_round u128_to_f64_truncate u128 u64 u128_to_f64);

impl_signed!(i8_to_f32 i8 u8_to_f32 u32);
impl_signed!(i16_to_f32 i16 u16_to_f32 u32);
impl_signed!(i32_to_f32_round i32 u32_to_f32_round u32);
impl_signed!(i32_to_f32_truncate i32 u32_to_f32_truncate u32);
impl_signed!(i64_to_f32_round i64 u64_to_f32_round u32);
impl_signed!(i64_to_f32_truncate i64 u64_to_f32_truncate u32);
impl_signed!(i128_to_f32_round i128 u128_to_f32_round u32);
impl_signed!(i128_to_f32_truncate i128 u128_to_f32_truncate u32);

impl_signed!(i8_to_f64 i8 u8_to_f64 u64);
impl_signed!(i16_to_f64 i16 u16_to_f64 u64);
impl_signed!(i32_to_f64 i32 u32_to_f64 u64);
impl_signed!(i64_to_f64_round i64 u64_to_f64_round u64);
impl_signed!(i64_to_f64_truncate i64 u64_to_f64_truncate u64);
impl_signed!(i128_to_f64_round i128 u128_to_f64_round u64);
impl_signed!(i128_to_f64_truncate i128 u128_to_f64_truncate u64);

macro_rules! impl_to_int {
    ($f:tt $s:tt $e:tt $n:tt $t:tt $u:tt $signed:tt) => {
        #[inline]
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
