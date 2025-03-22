use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::{preferences::*, *};
use rand::{rngs::StdRng, SeedableRng};

pub fn preference_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("preferences");
    const VOTER_COUNT: usize = 1_000;

    let candidate_pool = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];
    let mut rng = StdRng::seed_from_u64(0);

    group.bench_function("impartial", |b| {
        b.iter(|| {
            let impartial = Impartial;
            let _ballots: Profile<Cardinal> =
                impartial.sample(&candidate_pool, VOTER_COUNT, &mut rng);
        })
    });

    group.bench_function("manual", |b| {
        b.iter(|| {
            let ballots: Profile<Ordinal> = vec![
                Ordinal(vec![0, 1, 2]),
                Ordinal(vec![1, 0, 2]),
                Ordinal(vec![2, 0, 1]),
            ]
            .into();
            let manual = Manual::new(ballots);
            let _ballots: Profile<Ordinal> = manual.sample(&candidate_pool, VOTER_COUNT, &mut rng);
        })
    });

    group.bench_function("plackett luce", |b| {
        b.iter(|| {
            let plackett_luce = PlackettLuce::new(vec![(0, 1.0), (1, 1.0), (2, 1.0)]);
            let _ballots: Profile<Ordinal> =
                plackett_luce.sample(&candidate_pool, VOTER_COUNT, &mut rng);
        })
    });

    group.bench_function("mallows", |b| {
        b.iter(|| {
            let mallows = Mallows::new(vec![0, 1, 2], 0.5);
            let _ballots: Profile<Ordinal> = mallows.sample(&candidate_pool, VOTER_COUNT, &mut rng);
        })
    });
}

criterion_group! {
    name = preferences;
    config = Criterion::default();
    targets = preference_benchmarks
}
criterion_main!(preferences);
