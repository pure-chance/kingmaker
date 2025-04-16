use criterion::{Criterion, criterion_group, criterion_main};
use kingmaker::prelude::{methods::*, preferences::*, tactics::*, *};
use rand::{SeedableRng, rngs::StdRng};

pub fn tactics_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("tactics");
    const VOTER_COUNT: usize = 1000;

    let candidates = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "B", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];
    let impartial = Impartial;
    let mut rng = StdRng::seed_from_u64(42);
    let _nominal_ballots: Profile<Nominal> = impartial.sample(&candidates, VOTER_COUNT, &mut rng);
    let ordinal_ballots: Profile<Ordinal> = impartial.sample(&candidates, VOTER_COUNT, &mut rng);
    let _cardinal_ballots: Profile<Cardinal> = impartial.sample(&candidates, VOTER_COUNT, &mut rng);

    group.bench_function("identity", |b| {
        b.iter(|| {
            let _strategic_ballots: Profile<<Plurality as Method>::Ballot> = ordinal_ballots
                .clone()
                .into_iter()
                .map(|ballot| Identity.apply(ballot))
                .collect::<Vec<_>>()
                .into();
        })
    });

    group.bench_function("compromise", |b| {
        b.iter(|| {
            let _strategic_ballots: Profile<<Plurality as Method>::Ballot> = ordinal_ballots
                .clone()
                .into_iter()
                .map(|ballot| Compromise(vec![0]).apply(ballot))
                .collect::<Vec<_>>()
                .into();
        })
    });

    group.bench_function("burial", |b| {
        b.iter(|| {
            let _strategic_ballots: Profile<<Plurality as Method>::Ballot> = ordinal_ballots
                .clone()
                .into_iter()
                .map(|ballot| Burial(vec![0]).apply(ballot))
                .collect::<Vec<_>>()
                .into();
        })
    });

    group.bench_function("pushover", |b| {
        b.iter(|| {
            let _strategic_ballots: Profile<<Plurality as Method>::Ballot> = ordinal_ballots
                .clone()
                .into_iter()
                .map(|ballot| Pushover::new(vec![0], vec![2]).apply(ballot))
                .collect::<Vec<_>>()
                .into();
        })
    });
}

criterion_group! {
    name = tactics;
    config = Criterion::default();
    targets = tactics_benchmarks
}
criterion_main!(tactics);
