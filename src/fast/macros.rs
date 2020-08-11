macro_rules! group {
    ($($x:tt)*) => { $($x)* };
}

macro_rules! impl_native {
    ($name:tt $from:tt $to:tt) => {
        #[inline]
        pub fn $name(x: $from) -> $to {
            x as $to
        }
    };
}

macro_rules! impl_soft {
    ($name:tt $from:tt $to:tt) => {
        /// Soft implementation.
        #[inline]
        pub fn $name(x: $from) -> $to {
            $to::from_bits(crate::soft::$name(x))
        }
    };
}

macro_rules! impl_special {
    ($name:tt $from:tt $to:tt) => {
        #[inline]
        pub fn $name(x: $from) -> $to {
            special::$name(x)
        }
    };
}
