//! Fast implementations of all conversion functions.
//!
//! These are implemented using a mix of native floating point instructions (if
//! available) and (partial) soft implementations.

#[allow(unused_macros)]
macro_rules! impl_native {
    ($name:tt $from:tt $to:tt) => {
        #[cfg_attr(not(noinline), inline)]
        pub fn $name(x: $from) -> $to {
            x as $to
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_soft {
    ($name:tt $from:tt $to:tt) => {
        /// Soft implementation.
        #[inline]
        pub fn $name(x: $from) -> $to {
            let x = impl_soft!(@to_bits $from x);
            let y = crate::soft::$name(x);
            impl_soft!(@from_bits $to y)
        }
    };
    (@from_bits f32 $x:tt) => { f32::from_bits($x) };
    (@from_bits f64 $x:tt) => { f64::from_bits($x) };
    (@from_bits $_:tt $x:tt) => { $x };
    (@to_bits f32 $x:tt) => { f32::to_bits($x) };
    (@to_bits f64 $x:tt) => { f64::to_bits($x) };
    (@to_bits $_:tt $x:tt) => { $x };
}

#[allow(unused_macros)]
macro_rules! impl_special {
    ($name:tt $from:tt $to:tt) => {
        #[cfg_attr(not(noinline), inline)]
        pub fn $name(x: $from) -> $to {
            crate::special::$name(x)
        }
    };
}

// TODO: Fast implementations.
impl_soft!(u32_to_f32_truncate u32 f32);
impl_soft!(i32_to_f32_truncate i32 f32);
impl_soft!(u64_to_f32_truncate u64 f32);
impl_soft!(i64_to_f32_truncate i64 f32);
impl_soft!(u128_to_f32_truncate u128 f32);
impl_soft!(i128_to_f32_truncate i128 f32);
impl_soft!(u64_to_f64_truncate u64 f64);
impl_soft!(i64_to_f64_truncate i64 f64);
impl_soft!(u128_to_f64_truncate u128 f64);
impl_soft!(i128_to_f64_truncate i128 f64);

#[cfg(target_arch = "aarch64")]
group! {
    impl_native!(f32_to_u32 f32 u32);
    impl_native!(f32_to_i32 f32 i32);
    impl_native!(f32_to_u64 f32 u64);
    impl_native!(f32_to_i64 f32 i64);
    impl_soft!(f32_to_u128 f32 u128);
    impl_soft!(f32_to_i128 f32 i128);
    impl_native!(f64_to_u32 f64 u32);
    impl_native!(f64_to_i32 f64 i32);
    impl_native!(f64_to_u64 f64 u64);
    impl_native!(f64_to_i64 f64 i64);
    impl_soft!(f64_to_u128 f64 u128);
    impl_soft!(f64_to_i128 f64 i128);
    impl_native!(u8_to_f32 u8 f32);
    impl_native!(i8_to_f32 i8 f32);
    impl_native!(u16_to_f32 u16 f32);
    impl_native!(i16_to_f32 i16 f32);
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_native!(u64_to_f32_round u64 f32);
    impl_native!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u8_to_f64 u8 f64);
    impl_native!(i8_to_f64 i8 f64);
    impl_native!(u16_to_f64 u16 f64);
    impl_native!(i16_to_f64 i16 f64);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_native!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_special!(u128_to_f64_round u128 f64);
    impl_special!(i128_to_f64_round i128 f64);
}

#[cfg(target_arch = "x86_64")]
group! {
    impl_native!(f32_to_u32 f32 u32);
    impl_native!(f32_to_i32 f32 i32);
    impl_native!(f32_to_u64 f32 u64);
    impl_native!(f32_to_i64 f32 i64);
    impl_soft!(f32_to_u128 f32 u128);
    impl_soft!(f32_to_i128 f32 i128);
    impl_native!(f64_to_u32 f64 u32);
    impl_native!(f64_to_i32 f64 i32);
    impl_native!(f64_to_u64 f64 u64);
    impl_native!(f64_to_i64 f64 i64);
    impl_soft!(f64_to_u128 f64 u128);
    impl_soft!(f64_to_i128 f64 i128);
    impl_native!(u8_to_f32 u8 f32);
    impl_native!(i8_to_f32 i8 f32);
    impl_native!(u16_to_f32 u16 f32);
    impl_native!(i16_to_f32 i16 f32);
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_native!(u64_to_f32_round u64 f32);
    impl_native!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u8_to_f64 u8 f64);
    impl_native!(i8_to_f64 i8 f64);
    impl_native!(u16_to_f64 u16 f64);
    impl_native!(i16_to_f64 i16 f64);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_native!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_special!(u128_to_f64_round u128 f64);
    impl_special!(i128_to_f64_round i128 f64);
}

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
group! {
    impl_native!(f32_to_u32 f32 u32);
    impl_native!(f32_to_i32 f32 i32);
    impl_native!(f32_to_u64 f32 u64);
    impl_native!(f32_to_i64 f32 i64);
    impl_soft!(f32_to_u128 f32 u128);
    impl_soft!(f32_to_i128 f32 i128);
    impl_native!(f64_to_u32 f64 u32);
    impl_native!(f64_to_i32 f64 i32);
    impl_native!(f64_to_u64 f64 u64);
    impl_native!(f64_to_i64 f64 i64);
    impl_soft!(f64_to_u128 f64 u128);
    impl_soft!(f64_to_i128 f64 i128);
    impl_native!(u8_to_f32 u8 f32);
    impl_native!(i8_to_f32 i8 f32);
    impl_native!(u16_to_f32 u16 f32);
    impl_native!(i16_to_f32 i16 f32);
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_special!(u64_to_f32_round u64 f32);
    impl_native!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u8_to_f64 u8 f64);
    impl_native!(i8_to_f64 i8 f64);
    impl_native!(u16_to_f64 u16 f64);
    impl_native!(i16_to_f64 i16 f64);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_special!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_special!(u128_to_f64_round u128 f64);
    impl_special!(i128_to_f64_round i128 f64);
}

#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
group! {
    impl_native!(f32_to_u32 f32 u32);
    impl_native!(f32_to_i32 f32 i32);
    impl_native!(f32_to_u64 f32 u64);
    impl_native!(f32_to_i64 f32 i64);
    impl_soft!(f32_to_u128 f32 u128);
    impl_soft!(f32_to_i128 f32 i128);
    impl_native!(f64_to_u32 f64 u32);
    impl_native!(f64_to_i32 f64 i32);
    impl_native!(f64_to_u64 f64 u64);
    impl_native!(f64_to_i64 f64 i64);
    impl_soft!(f64_to_u128 f64 u128);
    impl_soft!(f64_to_i128 f64 i128);
    impl_native!(u8_to_f32 u8 f32);
    impl_native!(i8_to_f32 i8 f32);
    impl_native!(u16_to_f32 u16 f32);
    impl_native!(i16_to_f32 i16 f32);
    impl_special!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_soft!(u64_to_f32_round u64 f32);
    impl_soft!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u8_to_f64 u8 f64);
    impl_native!(i8_to_f64 i8 f64);
    impl_native!(u16_to_f64 u16 f64);
    impl_native!(i16_to_f64 i16 f64);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_soft!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_soft!(u128_to_f64_round u128 f64);
    impl_soft!(i128_to_f64_round i128 f64);
}

#[cfg(target_feature = "vfp2")]
group! {
    impl_native!(f32_to_u32 f32 u32);
    impl_native!(f32_to_i32 f32 i32);
    impl_soft!(f32_to_u64 f32 u64);
    impl_soft!(f32_to_i64 f32 i64);
    impl_soft!(f32_to_u128 f32 u128);
    impl_soft!(f32_to_i128 f32 i128);
    impl_native!(f64_to_u32 f64 u32);
    impl_native!(f64_to_i32 f64 i32);
    impl_soft!(f64_to_u64 f64 u64);
    impl_soft!(f64_to_i64 f64 i64);
    impl_soft!(f64_to_u128 f64 u128);
    impl_soft!(f64_to_i128 f64 i128);
    impl_native!(u8_to_f32 u8 f32);
    impl_native!(i8_to_f32 i8 f32);
    impl_native!(u16_to_f32 u16 f32);
    impl_native!(i16_to_f32 i16 f32);
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_soft!(u64_to_f32_round u64 f32);
    impl_soft!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u8_to_f64 u8 f64);
    impl_native!(i8_to_f64 i8 f64);
    impl_native!(u16_to_f64 u16 f64);
    impl_native!(i16_to_f64 i16 f64);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_soft!(u64_to_f64_round u64 f64);
    impl_soft!(i64_to_f64_round i64 f64);
    impl_soft!(u128_to_f64_round u128 f64);
    impl_soft!(i128_to_f64_round i128 f64);
}

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "x86_64",
    target_arch = "x86",
    target_feature = "vfp2",
)))]
group! {
    impl_soft!(f32_to_u32 f32 u32);
    impl_soft!(f32_to_i32 f32 i32);
    impl_soft!(f32_to_u64 f32 u64);
    impl_soft!(f32_to_i64 f32 i64);
    impl_soft!(f32_to_u128 f32 u128);
    impl_soft!(f32_to_i128 f32 i128);
    impl_soft!(f64_to_u32 f64 u32);
    impl_soft!(f64_to_i32 f64 i32);
    impl_soft!(f64_to_u64 f64 u64);
    impl_soft!(f64_to_i64 f64 i64);
    impl_soft!(f64_to_u128 f64 u128);
    impl_soft!(f64_to_i128 f64 i128);
    impl_soft!(u8_to_f32 u8 f32);
    impl_soft!(i8_to_f32 i8 f32);
    impl_soft!(u16_to_f32 u16 f32);
    impl_soft!(i16_to_f32 i16 f32);
    impl_soft!(u32_to_f32_round u32 f32);
    impl_soft!(i32_to_f32_round i32 f32);
    impl_soft!(u64_to_f32_round u64 f32);
    impl_soft!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_soft!(u8_to_f64 u8 f64);
    impl_soft!(i8_to_f64 i8 f64);
    impl_soft!(u16_to_f64 u16 f64);
    impl_soft!(i16_to_f64 i16 f64);
    impl_soft!(u32_to_f64 u32 f64);
    impl_soft!(i32_to_f64 i32 f64);
    impl_soft!(u64_to_f64_round u64 f64);
    impl_soft!(i64_to_f64_round i64 f64);
    impl_soft!(u128_to_f64_round u128 f64);
    impl_soft!(i128_to_f64_round i128 f64);
}
