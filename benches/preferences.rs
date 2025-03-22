use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

pub fn preference_benchmarks(c: &mut Criterion) {
    let mut _group = c.benchmark_group("preferences");
    const _VOTER_COUNT: usize = 1_000;

    let _candidate_pool = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];
    let mut _rng = StdRng::seed_from_u64(0);
}

criterion_group! {
    name = preferences;
    config = Criterion::default();
    targets = preference_benchmarks
}
criterion_main!(preferences);
