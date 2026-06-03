use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tt_crypto_primitives::{keccak256, sha256};

fn bench_keccak256(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccak256");

    for size in [32, 64, 128, 256, 1024, 4096] {
        let input: Vec<u8> = (0..size).map(|i| (i * 0x9e + 0x37) as u8).collect();
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &input,
            |b, input| b.iter(|| keccak256(black_box(input))),
        );
    }

    group.finish();
}

fn bench_sha256(c: &mut Criterion) {
    let mut group = c.benchmark_group("sha256");

    for size in [32, 64, 128, 256, 1024, 4096] {
        let input: Vec<u8> = (0..size).map(|i| (i * 0x9e + 0x37) as u8).collect();
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &input,
            |b, input| b.iter(|| sha256(black_box(input))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_keccak256, bench_sha256);
criterion_main!(benches);
