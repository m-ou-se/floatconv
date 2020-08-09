macro_rules! group {
    ($($x:tt)*) => { $($x)* };
}

macro_rules! impl_native {
    ($name:tt $from:tt $to:tt) => {
        pub fn $name(x: $from) -> $to {
            x as $to
        }
    };
}

macro_rules! impl_soft {
    ($name:tt $from:tt $to:tt) => {
        /// Soft implementation.
        pub fn $name(x: $from) -> $to {
            $to::from_bits(crate::soft::$name(x))
        }
    };
}

macro_rules! impl_signed {
    ($name:tt $from:tt $bits:tt $unsigned:tt) => {
        pub fn $name(x: $from) -> f64 {
            let s = ((x >> $bits - 1) as u64) << 63;
            f64::from_bits($unsigned(x.wrapping_abs() as _).to_bits() | s)
        }
    };
}
