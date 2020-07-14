use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use data_oriented_example::{gen_dop, gen_oop, run_dop, run_oop};

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("ApplyMotion");
    group.warm_up_time(Duration::from_millis(1000));
    group.measurement_time(Duration::from_millis(15000));
    group.sample_size(10);

    for i in [
        10, 100, 200, 500, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ]
    .into_iter()
    {
        let mut oop_input = gen_oop(*i);
        let mut dop_input = gen_dop(*i);

        group.bench_with_input(BenchmarkId::new("OOP", i), i, |b, i| {
            b.iter(|| run_oop(&mut oop_input))
        });
        group.bench_with_input(BenchmarkId::new("DOP", i), i, |b, i| {
            b.iter(|| run_dop(&mut dop_input))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
