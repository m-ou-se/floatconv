//! Specialized implementations for specific targets.

#[allow(unused_macros)]
macro_rules! impl_signed {
    ($name:tt $from:tt $bits:tt $unsigned:tt) => {
        pub fn $name(x: $from) -> f64 {
            let s = ((x >> $bits - 1) as u64) << 63;
            f64::from_bits($unsigned(x.wrapping_abs() as _).to_bits() | s)
        }
    };
}

#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
pub fn u32_to_f32_round(mut x: u32) -> f32 {
    if x >> 31 > 0 {
        x = x >> 1 | x & 1;
    }
    x as i32 as f32
}

#[cfg(target_feature = "sse2")]
pub fn u64_to_f32_round(mut x: u64) -> f32 {
    if x >> 63 > 0 {
        x = x >> 1 | x & 1;
    }
    x as i64 as f32
}

#[cfg(target_feature = "sse2")]
pub fn u64_to_f64_round(x: u64) -> f64 {
    const A: f64 = (1u128 << 52) as f64;
    const B: f64 = (1u128 << 84) as f64;
    let l = f64::from_bits(A.to_bits() | x << 32 >> 32) - A;
    let h = f64::from_bits(B.to_bits() | x >> 32) - B;
    l + h
}

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
