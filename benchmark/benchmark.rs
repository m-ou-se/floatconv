use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use floatconv::*;

fn bench_u128_to_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("u128_to_f64");
    let inputs = &[
        (u128::max_value(), "u128-max"),
        (1234, "1234"),
        (1234 << 80, "1234-80zeros"),
    ];
    for &(input, name) in inputs {
        group.bench_with_input(BenchmarkId::new("semi", name), &input, |b, &x| {
            b.iter(|| floatconv::semi::u128_to_f64_round(black_box(x)))
        });
    }
    for &(input, name) in inputs {
        group.bench_with_input(BenchmarkId::new("soft", name), &input, |b, &x| {
            b.iter(|| floatconv::soft::u128_to_f64_round(black_box(x)))
        });
    }
    for &(input, name) in inputs {
        group.bench_with_input(BenchmarkId::new("builtin", name), &input, |b, &x| {
            b.iter(|| black_box(x) as f64)
        });
    }
    group.finish();
}

criterion_group!(benches, bench_u128_to_f64);
criterion_main!(benches);
