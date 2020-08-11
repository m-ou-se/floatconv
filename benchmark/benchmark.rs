use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! benches {
    ($c:ident $f:ident $t:ident $t2:ident $inputs:ident) => {{
        #[inline(never)]
        fn soft_conv(x: $t) -> $t2 { $t2::from_bits(floatconv::soft::$f(x)) }
        #[inline(never)]
        fn fast_conv(x: $t) -> $t2 { floatconv::fast::$f(x) }
        #[inline(never)]
        fn builtin_conv(x: $t) -> $t2 { x as $t2 }
        let mut group = $c.benchmark_group(stringify!($f));
        for &(input, name) in $inputs {
            group.bench_with_input(BenchmarkId::new("soft", name), &input, |b, &x| {
                b.iter(|| soft_conv(black_box(x)))
            });
        }
        for &(input, name) in $inputs {
            group.bench_with_input(BenchmarkId::new("fast", name), &input, |b, &x| {
                b.iter(|| fast_conv(black_box(x)))
            });
        }
        for &(input, name) in $inputs {
            group.bench_with_input(BenchmarkId::new("builtin", name), &input, |b, &x| {
                b.iter(|| builtin_conv(black_box(x)));
            });
        }
        group.finish();
    }};
}

fn bench_u32(c: &mut Criterion) {
    let inputs = &[
        //(0, "zero"),
        //(u32::max_value(), "max"),
        //(1234, "1234"),
        (1234u32 << 20 | 4321, "some-number"),
    ];
    benches!(c u32_to_f32_round u32 f32 inputs);
    benches!(c u32_to_f64 u32 f64 inputs);
}

fn bench_i32(c: &mut Criterion) {
    let inputs = &[
        //(0, "zero"),
        //(i32::max_value(), "max"),
        //(1234, "1234"),
        (-1234i32 << 20 | 4321, "some-number"),
    ];
    benches!(c i32_to_f32_round i32 f32 inputs);
    benches!(c i32_to_f64 i32 f64 inputs);
}

fn bench_u64(c: &mut Criterion) {
    let inputs = &[
        //(0, "zero"),
        //(u64::max_value(), "max"),
        //(1234, "1234"),
        (1234u64 << 45 | 4321, "some-number"),
    ];
    benches!(c u64_to_f32_round u64 f32 inputs);
    benches!(c u64_to_f64_round u64 f64 inputs);
}

fn bench_i64(c: &mut Criterion) {
    let inputs = &[
        //(0, "zero"),
        //(i64::max_value(), "max"),
        //(1234, "1234"),
        (-1234i64 << 45 | 4321, "some-number"),
    ];
    benches!(c i64_to_f32_round i64 f32 inputs);
    benches!(c i64_to_f64_round i64 f64 inputs);
}

fn bench_u128(c: &mut Criterion) {
    let inputs = &[
        //(0, "zero"),
        //(u128::max_value(), "max"),
        //(1234, "1234"),
        (1234u128 << 80 | 4321, "some-number"),
    ];
    benches!(c u128_to_f32_round u128 f32 inputs);
    benches!(c u128_to_f64_round u128 f64 inputs);
}

fn bench_i128(c: &mut Criterion) {
    let inputs = &[
        //(0, "zero"),
        //(i128::max_value(), "max"),
        //(1234, "1234"),
        (-1234i128 << 80 | 4321, "some-number"),
    ];
    benches!(c i128_to_f32_round i128 f32 inputs);
    benches!(c i128_to_f64_round i128 f64 inputs);
}

criterion_group!(benches,
    bench_u32,
    bench_i32,
    bench_u64,
    bench_i64,
    bench_u128,
    bench_i128,
);
criterion_main!(benches);
