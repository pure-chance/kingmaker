use criterion::{Criterion, criterion_group, criterion_main};
use kingmaker::prelude::{methods::*, preferences::*, *};
use rand::{SeedableRng, rngs::StdRng};

pub fn method_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("methods");
    const VOTER_COUNT: usize = 1_000;

    let candidates = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];

    let impartial = Impartial;
    let mut rng = StdRng::seed_from_u64(0);
    let nominal_ballots: Profile<Nominal> = impartial.sample(&candidates, 1_000, &mut rng);
    let ordinal_ballots: Profile<Ordinal> = impartial.sample(&candidates, VOTER_COUNT, &mut rng);
    let cardinal_ballots: Profile<Cardinal> = impartial.sample(&candidates, VOTER_COUNT, &mut rng);

    group.bench_function("random dictator", |b| {
        b.iter(|| {
            let _outcome = RandomDictator.outcome(&candidates, Profile::clone(&ordinal_ballots));
        })
    });

    group.bench_function("plurality", |b| {
        b.iter(|| {
            let _outcome = Plurality.outcome(&candidates, Profile::clone(&ordinal_ballots));
        })
    });

    group.bench_function("approval", |b| {
        b.iter(|| {
            let _outcome = Approval.outcome(&candidates, Profile::clone(&nominal_ballots));
        })
    });

    group.bench_function("borda", |b| {
        b.iter(|| {
            let _outcome = Borda.outcome(&candidates, Profile::clone(&ordinal_ballots));
        })
    });

    group.bench_function("star", |b| {
        b.iter(|| {
            let _outcome = Star.outcome(&candidates, Profile::clone(&cardinal_ballots));
        })
    });

    group.bench_function("instant runoff", |b| {
        b.iter(|| {
            let _outcome = IRV.outcome(&candidates, Profile::clone(&ordinal_ballots));
        })
    });

    group.bench_function("single transferable vote", |b| {
        b.iter(|| {
            let _outcome = STV::new(2).outcome(&candidates, Profile::clone(&ordinal_ballots));
        })
    });
}

criterion_group! {
    name = methods;
    config = Criterion::default();
    targets = method_benchmarks
}
criterion_main!(methods);
