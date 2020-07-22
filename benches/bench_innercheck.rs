use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use data_oriented_example::{gen_mixed, gen_separate, run_mixed, run_separate};

fn bench_innercheck(c: &mut Criterion) {
    let mut group = c.benchmark_group("InnerCheck");
    group.warm_up_time(Duration::from_millis(1000));
    group.measurement_time(Duration::from_millis(15000));
    group.sample_size(10);

    for i in [
        100, 1000, 2000, 5000, 10000, 50000, 100000, 1000000, 3000000, 5000000,
    ]
    .into_iter()
    {
        let mixed = gen_mixed(*i);
        let separate = gen_separate(*i);

        group.bench_with_input(BenchmarkId::new("Mixed", i), i, |b, i| {
            b.iter(|| run_mixed(&mixed))
        });
        group.bench_with_input(BenchmarkId::new("Separate", i), i, |b, i| {
            b.iter(|| run_separate(&separate.0, &separate.1, &separate.2))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_innercheck);
criterion_main!(benches);
