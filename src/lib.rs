pub mod emulated;

pub mod native {
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn u32_to_f64(x: u32) -> f64 { x as f64 }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn u64_to_f64_round(x: u64) -> f64 { x as f64 }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn i32_to_f64(x: i32) -> f64 { x as f64 }

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn i64_to_f64_round(x: i64) -> f64 { x as f64 }
}

pub use emulated::{
    i128_to_f64_round, i128_to_f64_truncate, u128_to_f64_round, u128_to_f64_truncate,
    u64_to_f64_truncate,
};

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
pub use emulated::{i32_to_f64, i64_to_f64_round, u32_to_f64, u64_to_f64_round};

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub use native::*;

#[cfg(test)]
mod test;
