//! Fast implementations of all conversion functions.
//!
//! These are implemented using a mix of native floating point instructions (if
//! available) and (partial) soft implementations.

#[macro_use]
#[allow(unused_macros)]
mod macros;

mod special;

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
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_native!(u64_to_f32_round u64 f32);
    impl_native!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_native!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_special!(u128_to_f64_round u128 f64);
    impl_special!(i128_to_f64_round i128 f64);
}

#[cfg(target_arch = "x86_64")]
group! {
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_special!(u64_to_f32_round u64 f32);
    impl_native!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_special!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_special!(u128_to_f64_round u128 f64);
    impl_special!(i128_to_f64_round i128 f64);
}

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
group! {
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_special!(u64_to_f32_round u64 f32);
    impl_native!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_special!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_special!(u128_to_f64_round u128 f64);
    impl_special!(i128_to_f64_round i128 f64);
}

#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
group! {
    impl_special!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_soft!(u64_to_f32_round u64 f32);
    impl_soft!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_native!(u32_to_f64 u32 f64);
    impl_native!(i32_to_f64 i32 f64);
    impl_soft!(u64_to_f64_round u64 f64);
    impl_native!(i64_to_f64_round i64 f64);
    impl_soft!(u128_to_f64_round u128 f64);
    impl_soft!(i128_to_f64_round i128 f64);
}

#[cfg(target_feature = "vfp2")]
group! {
    impl_native!(u32_to_f32_round u32 f32);
    impl_native!(i32_to_f32_round i32 f32);
    impl_soft!(u64_to_f32_round u64 f32);
    impl_soft!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
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
    impl_soft!(u32_to_f32_round u32 f32);
    impl_soft!(i32_to_f32_round i32 f32);
    impl_soft!(u64_to_f32_round u64 f32);
    impl_soft!(i64_to_f32_round i64 f32);
    impl_soft!(u128_to_f32_round u128 f32);
    impl_soft!(i128_to_f32_round i128 f32);
    impl_soft!(u32_to_f64 u32 f64);
    impl_soft!(i32_to_f64 i32 f64);
    impl_soft!(u64_to_f64_round i64 f64);
    impl_soft!(i64_to_f64_round i64 f64);
    impl_soft!(u128_to_f64_round u128 f64);
    impl_soft!(i128_to_f64_round i128 f64);
}
