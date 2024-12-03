use criterion::{criterion_group, criterion_main, Criterion};

fn basic_benchmark(c: &mut Criterion) {
    c.bench_function("simple_addition", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });
    });
}

criterion_group!(benches, basic_benchmark);
criterion_main!(benches);
