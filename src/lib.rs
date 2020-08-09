//! Floating point conversion functions.
//!
//! ## Software and hardware implementations
//!
//! The [`native`] module provides only those conversions which are natively
//! available on the target hardware.
//!
//! The [`soft`] module provides a software implementation of all
//! conversion functions, for targets which do not provide them natively.
//! These are implemented without any floating point operations.
//!
//! The [`semi`] module provides a software implementation of some conversion
//! functions which are not natively available on the target, but are
//! implemented using other native floating point operations.
//!
//! The root of the crate exposes the best version of all available functions.
//!
//! ## Rounding mode
//!
//! - Functions named `_round` round the integer to the closest possible floating point number, and breaks ties to even.
//! - Functions named `_truncate` truncate the result, which means they round towards zero.
//! - Functions named `_any` are an alias of whichever variant is fastest on the target.
//! - Functions without a rounding mode in their name do not round. These conversions are always lossless.
//!
//! ## Speed
//!
//! For conversions that aren't available natively, the software
//! implementations in this crate seem to be both faster and smaller in almost
//! all cases compared to the ones currently used by `x as f64` (from the
//! compiler builtins runtime support library).
//!
//! ## Work in progress
//!
//! This crate is usable, but still incomplete:
//!
//! - There's no support for `f32` yet.
//! - There's no support for converting *to* integers yet.
//! - Native conversions are only available on ARM (32- and 64-bit) and x86 (32- and 64-bit).
//! - The software implementations can probably be optimized further.
//! - More benchmarking still needs to happen.

macro_rules! impl_signed {
    (
        $name:tt
        $from:tt
        $bits:tt
        $unsigned:tt
    ) => {
        pub fn $name(x: $from) -> f64 {
            let s = ((x >> $bits - 1) as u64) << 63;
            f64::from_bits($unsigned(x.wrapping_abs() as _).to_bits() | s)
        }
    };
}

/// Software implementations of all conversion functions that don't use any
/// floating point operations.
///
/// Available on all platforms.
pub mod soft;

/// Software implementations of some conversion functions that make use of
/// other native floating point operations.
///
/// **Please note:** The contents of this module varies by target.
/// Functions available on one target might not exist on another.
pub mod semi;

/// Floating point conversion instructions natively available on the target.
///
/// **Please note:** The contents of this module varies by target.
/// Functions available on one target might not exist on another.
pub mod native {
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64",
        target_arch = "x86",
        target_feature = "vfp2"
    ))]
    pub fn u32_to_f64(x: u32) -> f64 {
        x as f64
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86"))]
    pub fn u64_to_f64_round(x: u64) -> f64 {
        x as f64
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64",
        target_arch = "x86",
        target_feature = "vfp2"
    ))]
    pub fn i32_to_f64(x: i32) -> f64 {
        x as f64
    }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86"))]
    pub fn i64_to_f64_round(x: i64) -> f64 {
        x as f64
    }
}

pub use soft::{
    i128_to_f64_truncate, i64_to_f64_truncate, u128_to_f64_truncate, u64_to_f64_truncate,
};

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64",))]
pub use semi::{
    i128_to_f64_round, i128_to_f64_round as i128_to_f64_any, u128_to_f64_round,
    u128_to_f64_round as u128_to_f64_any,
};

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64",)))]
pub use soft::{
    i128_to_f64_round, i128_to_f64_truncate as i128_to_f64_any, u128_to_f64_round,
    u128_to_f64_truncate as u128_to_f64_any,
};

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86_64",
    target_arch = "x86",
    target_feature = "vfp2"
))]
pub use native::{i32_to_f64, u32_to_f64};

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86"))]
pub use native::{
    i64_to_f64_round, i64_to_f64_round as i64_to_f64_any, u64_to_f64_round,
    u64_to_f64_round as u64_to_f64_any,
};

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "x86_64",
    target_arch = "x86",
    target_feature = "vfp2"
)))]
pub use soft::{i32_to_f64, u32_to_f64};

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")))]
pub use soft::{
    i64_to_f64_round, i64_to_f64_truncate as i64_to_f64_any, u64_to_f64_round,
    u64_to_f64_truncate as u64_to_f64_any,
};

#[cfg(test)]
mod test;
