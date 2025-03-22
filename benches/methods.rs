use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::{methods::*, preferences::*, *};
use rand::{rngs::StdRng, SeedableRng};

pub fn method_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("methods");
    const VOTER_COUNT: usize = 1_000;

    let candidate_pool = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];

    let impartial = Impartial;
    let mut rng = StdRng::seed_from_u64(0);
    let nominal_ballots: Profile<Nominal> = impartial.sample(&candidate_pool, 1_000, &mut rng);
    let ordinal_ballots: Profile<Ordinal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
    let cardinal_ballots: Profile<Cardinal> =
        impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);

    group.bench_function("random dictator", |b| {
        b.iter(|| {
            let _outcome = RandomDictator.outcome(&candidate_pool, &ordinal_ballots);
        })
    });

    group.bench_function("plurality", |b| {
        b.iter(|| {
            let _outcome = Plurality.outcome(&candidate_pool, &ordinal_ballots);
        })
    });

    group.bench_function("approval", |b| {
        b.iter(|| {
            let _outcome = Approval.outcome(&candidate_pool, &nominal_ballots);
        })
    });

    group.bench_function("borda", |b| {
        b.iter(|| {
            let _outcome = Borda.outcome(&candidate_pool, &ordinal_ballots);
        })
    });

    group.bench_function("star", |b| {
        b.iter(|| {
            let _outcome = Star.outcome(&candidate_pool, &cardinal_ballots);
        })
    });

    group.bench_function("instant runoff", |b| {
        b.iter(|| {
            let _outcome = IRV.outcome(&candidate_pool, &ordinal_ballots);
        })
    });

    group.bench_function("single transferable vote", |b| {
        b.iter(|| {
            let _outcome = STV::new(2).outcome(&candidate_pool, &ordinal_ballots);
        })
    });
}

criterion_group! {
    name = methods;
    config = Criterion::default();
    targets = method_benchmarks
}
criterion_main!(methods);
