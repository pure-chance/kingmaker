use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::{preferences::*, *};
use rand::{rngs::StdRng, SeedableRng};

pub fn tactics_benchmarks(c: &mut Criterion) {
    let mut _group = c.benchmark_group("tactics");
    const VOTER_COUNT: usize = 1000;

    let candidate_pool = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];
    let impartial = Impartial;
    let mut rng = StdRng::seed_from_u64(42);
    let _nominal_ballots: Profile<Nominal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
    let ordinal_ballots: Profile<Ordinal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
    let _cardinal_ballots: Profile<Cardinal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
}

criterion_group! {
    name = tactics;
    config = Criterion::default();
    targets = tactics_benchmarks
}
criterion_main!(tactics);
