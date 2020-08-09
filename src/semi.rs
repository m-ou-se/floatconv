#[cfg(any(target_arch = "aarch64", target_arch = "x86_64",))]
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

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64",))]
impl_signed!(i128_to_f64_round i128 128 u128_to_f64_round);
