#![cfg_attr(not(test), no_std)]

//! Floating point conversion functions.
//!
//! ## Software and hardware implementations
//!
//! The [`soft`] module provides a software implementation of all
//! conversion functions, for targets which do not provide them natively.
//! These are implemented without any floating point operations, so are also
//! useful for software that needs to avoid using floating point hardware.
//!
//! The [`fast`] module provides a fast implementation of all conversion
//! functions by making use of native floating point instructions where
//! possible.
//!
//! ## Conversion of integers to floating point values
//!
//! - Functions named `_round` round the integer to the closest possible
//!   floating point number, and breaks ties to even.
//! - Functions named `_truncate` truncate the result, which means they round
//!   towards zero.
//! - Functions without a rounding mode in their name do not round. These
//!   conversions are always lossless.
//! - The only conversion that can overflow is `u128_to_f32_round`, in which
//!   case it returns `f32::INFINITY`.
//!
//! ## Conversion of floating point values to integers
//!
//! - These conversions truncate, which means they round towards zero.
//! - Values higher than what the integer can represent (including +∞) result
//!   in the maximum integer value.
//! - Values lower than what the integer can represent (including −∞) result
//!   in the minimum integer value.
//! - `NaN` is converted to zero.
//!
//! ## Speed
//!
//! For conversions that aren't available natively, the software
//! implementations in this crate seem to be both faster and smaller in almost
//! all cases compared to the ones currently used by `x as <type>` (from the
//! compiler builtins runtime support library).
//!
//! ## Work in progress
//!
//! This crate is usable, but still incomplete:
//!
//! - Native conversions are only available on ARM (32- and 64-bit) and x86 (32- and 64-bit).
//! - The truncating functions do not (yet) use any native floating point instructions.

// Used to group items together for #[cfg(..)].
macro_rules! group {
    ($($x:tt)*) => { $($x)* };
}

#[cfg(test)]
mod test;

pub mod soft;

pub mod fast;

mod special;
