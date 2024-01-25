use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_iteration_benchmarks::{Logic, Thing};

fn one_million_things() -> Logic {
    let mut logic = Logic::new();
    for i in 0..1_000_000 {
        logic.add_thing(Thing::new(i, format!("Thing {}", i)));
    }
    logic
}

fn do_something_benchmark(c: &mut Criterion) {
    let mut logic = one_million_things();
    c.bench_function("do_something", |b| b.iter(|| {
        // Your code to benchmark goes here
        let b4 = logic.count();
        black_box(logic.do_something());
        assert!(b4!=logic.count());
    }));
}

fn do_something_odd_filter_benchmark(c: &mut Criterion) {
    let mut logic = one_million_things();
    c.bench_function("do_something_odd_filter", |b| b.iter(|| {
        // Your code to benchmark goes here
        let b4 = logic.count();
        black_box(logic.do_something_odd_filter());
        assert!(b4!=logic.count());
    }));
}

fn do_something_external_odd_filter_benchmark(c: &mut Criterion) {
    let mut logic = one_million_things();
    c.bench_function("do_something_external_odd_filter", |b| b.iter(|| {
        // Your code to benchmark goes here
        let b4 = logic.count();
        black_box(logic.do_something_external_odd_filter());
        assert!(b4!=logic.count());
    }));
}

criterion_group!(benches, do_something_benchmark, do_something_odd_filter_benchmark, do_something_external_odd_filter_benchmark);
criterion_main!(benches);
