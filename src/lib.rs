#![no_std]

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
//! ## Rounding mode
//!
//! - Functions named `_round` round the integer to the closest possible floating point number, and breaks ties to even.
//! - Functions named `_truncate` truncate the result, which means they round towards zero.
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
//! - The truncating functions do not (yet) use any native floating point instructions.
//! - More benchmarking still needs to happen.

#[cfg(test)]
mod test;

pub mod soft;

pub mod fast;
