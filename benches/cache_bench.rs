use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use ttl_cache::Cache;

fn bench_insert(c: &mut Criterion) {
    c.bench_function("insert", |b| {
        b.iter(|| {
            let mut cache = Cache::<i32, i32>::new();
            cache.insert(black_box(1), black_box(2), Duration::from_secs(10));
        })
    });
}

fn bench_get_hit(c: &mut Criterion) {
    let mut cache = Cache::<i32, i32>::new();
    cache.insert(1, 2, Duration::from_secs(10));

    c.bench_function("get_hit", |b| {
        b.iter(|| {
            cache.get(&1);
        })
    });
}

fn bench_get_miss(c: &mut Criterion) {
    let mut cache = Cache::<i32, i32>::new();

    c.bench_function("get_miss", |b| {
        b.iter(|| {
            cache.get(&1);
        })
    });
}

criterion_group!(benches, bench_insert, bench_get_hit, bench_get_miss);
criterion_main!(benches);