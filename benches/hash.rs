use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let message = &[1, 2, 3, 4, 5];
    c.bench_function("hash data", |b| {
        b.iter(|| whirlpool::core::hash(message.clone().into()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main! {benches}
