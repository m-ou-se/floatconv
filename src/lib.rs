//! Floating point conversion functions.
//!
//! ## Emulated and Native Conversions
//!
//! The [`native`] module provides only those functions which are natively available on the target.
//!
//! The [`emulated`] module provides an emulated implementation of all
//! conversion functions, for targets which do not provide them natively.
//!
//! The root of the crate exposes the best version of all available functions.
//!
//! ## Rounding mode
//!
//! - Functions named `_round` round the integer to the closest possible floating point number, and breaks ties to even.
//! - Functions named `_truncate` truncate the result, which means they round towards zero.
//! - Functions without a rounding mode in their name do not round. These conversions are always lossless.

/// Floating point conversion functions emulated without floating point instructions.
///
/// Available on all platforms.
pub mod emulated;

/// Floating point conversion instructions natively available on the target.
///
/// **Please note:** The contents of this module varies by target.
/// Functions available on one target might not exist on another.
pub mod native {
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86", target_feature = "vfp2"))]
    pub fn u32_to_f64(x: u32) -> f64 { x as f64 }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86"))]
    pub fn u64_to_f64_round(x: u64) -> f64 { x as f64 }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86", target_feature = "vfp2"))]
    pub fn i32_to_f64(x: i32) -> f64 { x as f64 }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86"))]
    pub fn i64_to_f64_round(x: i64) -> f64 { x as f64 }
}

pub use emulated::{
    i128_to_f64_round, i128_to_f64_truncate, u128_to_f64_round, u128_to_f64_truncate,
    u64_to_f64_truncate,
};

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86", target_feature = "vfp2"))]
pub use native::{i32_to_f64, u32_to_f64};

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86"))]
pub use native::{i64_to_f64_round, u64_to_f64_round};

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86", target_feature = "vfp2")))]
pub use emulated::{i32_to_f64, u32_to_f64};

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "x86")))]
pub use emulated::{i64_to_f64_round, u64_to_f64_round};

#[cfg(test)]
mod test;
