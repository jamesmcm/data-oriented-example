use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use data_oriented_example::{gen_list, gen_vec, run_list, run_vec};

fn bench_linkedlist(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList");
    group.warm_up_time(Duration::from_millis(1000));
    group.measurement_time(Duration::from_millis(15000));
    group.sample_size(100);

    for i in [
        1000, 5000, 10000, 20000, 50000, 100000, 500000, 1000000, 3000000, 5000000,
    ]
    .iter()
    {
        let mut ll = gen_list(*i);
        let mut v = gen_vec(*i);

        group.bench_with_input(BenchmarkId::new("LinkedList", i), i, |b, _i| {
            b.iter(|| run_list(&mut ll))
        });
        group.bench_with_input(BenchmarkId::new("Vector", i), i, |b, _i| {
            b.iter(|| run_vec(&mut v))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_linkedlist);
criterion_main!(benches);
