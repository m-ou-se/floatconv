#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: u128| {
    assert_eq!(floatconv::fast::u128_to_f64_round(data), data as f64);
});
