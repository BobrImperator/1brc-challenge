use brc_impl::read_and_calculated_measuerements;
use criterion::{Criterion, criterion_group, criterion_main};

// Not a good idea :)
// Will add smaller benchmarks later
fn full_run_benchmark(c: &mut Criterion) {
    c.bench_function("full_run", |b| {
        b.iter(|| read_and_calculated_measuerements())
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = full_run_benchmark
);
criterion_main!(benches);
