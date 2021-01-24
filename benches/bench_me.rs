

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use npv::npv;


fn add_benchmark(c: &mut Criterion) {
    let mortality_rates = vec![0.001, 0.002, 0.003, 0.003, 0.004, 0.004, 0.005, 0.007, 0.009, 0.011];
    let lapse_rates = vec![0.05, 0.07, 0.08, 0.10, 0.14, 0.20, 0.20, 0.20, 0.10, 0.04];
    let premium = 100.0;
    let sum_assured = 25000.0;
    let interest_rate = 0.02;
    c.bench_function("npv", |b| b.iter(|| npv(&mortality_rates, &lapse_rates, interest_rate, sum_assured, premium, 1.0, None)));
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
