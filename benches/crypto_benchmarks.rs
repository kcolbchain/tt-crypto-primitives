use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tt_crypto_primitives::{
    keccak256, multiply_polynomials, multiply_polynomials_naive, sha256, Polynomial, N, Q,
};

fn bench_keccak256(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccak256");

    for size in [32, 64, 128, 256, 1024, 4096] {
        let input: Vec<u8> = (0..size).map(|i| (i * 0x9e + 0x37) as u8).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| keccak256(black_box(input)))
        });
    }

    group.finish();
}

fn bench_sha256(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256");

    for size in [32, 64, 128, 256, 1024, 4096] {
        let input: Vec<u8> = (0..size).map(|i| (i * 0x9e + 0x37) as u8).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| sha256(black_box(input)))
        });
    }

    group.finish();
}

fn bench_polynomial_multiply(c: &mut Criterion) {
    let lhs: Polynomial = core::array::from_fn(|i| ((i * i + 31 * i + 7) % Q as usize) as u16);
    let rhs: Polynomial = core::array::from_fn(|i| ((17 * i + 19) % Q as usize) as u16);

    let mut group = c.benchmark_group("polynomial_multiply_256");
    group.bench_function(BenchmarkId::new("ntt", N), |b| {
        b.iter(|| multiply_polynomials(black_box(&lhs), black_box(&rhs)))
    });
    group.bench_function(BenchmarkId::new("naive", N), |b| {
        b.iter(|| multiply_polynomials_naive(black_box(&lhs), black_box(&rhs)))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_keccak256,
    bench_sha256,
    bench_polynomial_multiply
);
criterion_main!(benches);
