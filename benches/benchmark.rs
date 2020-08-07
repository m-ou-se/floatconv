use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use floatconv::*;

fn bench_u128_to_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("u128_to_f64");
    for input in &[
        0,
        1,
        1234u128,
        1234u128 << 80,
        1u128 << 127,
        u128::max_value(),
    ] {
        group.bench_with_input(BenchmarkId::new("floatconv", input), input, |b, &x| {
            b.iter(|| u128_to_f64_round(black_box(x)))
        });
        group.bench_with_input(BenchmarkId::new("builtin", input), input, |b, &x| {
            b.iter(|| black_box(x) as f64)
        });
    }
    group.finish();
}

criterion_group!(benches, bench_u128_to_f64);
criterion_main!(benches);
