mod elections;
mod methods;
mod preferences;
mod tactics;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    preferences::preference_benchmarks,
    methods::method_benchmarks,
    elections::election_benchmarks,
    tactics::tactics_benchmarks,
);
criterion_main!(benches);
