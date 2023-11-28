use criterion::{criterion_group, criterion_main, Criterion};
use rusttracer::main_tb;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    group.bench_function("my-function", |b| b.iter(main_tb));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
