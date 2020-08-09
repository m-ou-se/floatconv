//! Fast implementations of all conversion functions.
//!
//! These are implemented using a mix of native floating point instructions (if
//! available) and (partial) soft implementations.

#[macro_use]
#[allow(unused_macros)]
mod macros;

// [f32]

// TODO: Fast implementations.
impl_soft!(u32_to_f32_round u32 f32);
impl_soft!(i32_to_f32_round i32 f32);
impl_soft!(u64_to_f32_round u64 f32);
impl_soft!(i64_to_f32_round i64 f32);
impl_soft!(u128_to_f32_round u128 f32);
impl_soft!(i128_to_f32_round i128 f32);
impl_soft!(u32_to_f32_truncate u32 f32);
impl_soft!(i32_to_f32_truncate i32 f32);
impl_soft!(u64_to_f32_truncate u64 f32);
impl_soft!(i64_to_f32_truncate i64 f32);
impl_soft!(u128_to_f32_truncate u128 f32);
impl_soft!(i128_to_f32_truncate i128 f32);

// 32-bit to f64 conversions

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86_64",
    all(target_arch = "x86", target_feature = "sse2"),
    target_feature = "vfp2"
))]
group! {
    impl_native!(u32_to_f64 u32);
    impl_native!(i32_to_f64 i32);
}

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "x86_64",
    all(target_arch = "x86", target_feature = "sse2"),
    target_feature = "vfp2"
)))]
group! {
    impl_soft!(u32_to_f64 u32 f64);
    impl_soft!(i32_to_f64 i32 f64);
}

// 64-bit to f64 conversions

#[cfg(target_arch = "aarch64")]
group! {
    impl_native!(u64_to_f64_round u64);
    impl_native!(i64_to_f64_round i64);
}

#[cfg(target_feature = "sse2")]
group! {
    pub fn u64_to_f64_round(x: u64) -> f64 {
        const A: f64 = (1u128 << 52) as f64;
        const B: f64 = (1u128 << 84) as f64;

        let l = f64::from_bits(A.to_bits() | x << 32 >> 32) - A;
        let h = f64::from_bits(B.to_bits() | x >> 32) - B;
        l + h
    }
}

#[cfg(not(any(target_arch = "aarch64", target_feature = "sse2")))]
impl_soft!(u64_to_f64_round u64 f64);

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
impl_native!(i64_to_f64_round i64);

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")))]
impl_soft!(i64_to_f64_round i64 f64);

impl_soft!(u64_to_f64_truncate u64 f64);
impl_soft!(i64_to_f64_truncate i64 f64);

// 128-bit to f64 conversions

#[cfg(any(target_arch = "aarch64", target_feature = "sse2"))]
group! {
    pub fn u128_to_f64_round(x: u128) -> f64 {
        const A: f64 = (1u128 << 52) as f64;
        const B: f64 = (1u128 << 104) as f64;
        const C: f64 = (1u128 << 76) as f64;
        const D: f64 = u128::MAX as f64;

        if x < 1 << 104 {
            let l = f64::from_bits(A.to_bits() | (x << 12) as u64 >> 12) - A;
            let h = f64::from_bits(B.to_bits() | (x >> 52) as u64) - B;
            l + h
        } else {
            let l = f64::from_bits(C.to_bits() | (x >> 12) as u64 >> 12 | x as u64 & 0xFFFFFF) - C;
            let h = f64::from_bits(D.to_bits() | (x >> 76) as u64) - D;
            l + h
        }
    }

    impl_signed!(i128_to_f64_round i128 128 u128_to_f64_round);
}

#[cfg(not(any(target_arch = "aarch64", target_feature = "sse2")))]
group! {
    impl_soft!(u128_to_f64_round u128 f64);
    impl_soft!(i128_to_f64_round i128 f64);
}

impl_soft!(u128_to_f64_truncate u128 f64);
impl_soft!(i128_to_f64_truncate i128 f64);
