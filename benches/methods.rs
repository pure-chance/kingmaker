use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::{preferences::*, *};
use rand::{rngs::StdRng, SeedableRng};

pub fn method_benchmarks(c: &mut Criterion) {
    let mut _group = c.benchmark_group("methods");
    const VOTER_COUNT: usize = 1_000;

    let candidate_pool = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];

    let impartial = Impartial;
    let mut rng = StdRng::seed_from_u64(0);
    let _nominal_ballots: Profile<Nominal> = impartial.sample(&candidate_pool, 1_000, &mut rng);
    let _ordinal_ballots: Profile<Ordinal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
    let _cardinal_ballots: Profile<Cardinal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
}

criterion_group! {
    name = methods;
    config = Criterion::default();
    targets = method_benchmarks
}
criterion_main!(methods);
