use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::*;

pub fn election_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("election");

    fn election_setup() -> Election<Ordinal, (), methods::Plurality> {
        // configure election
        let candidate_pool = vec![
            Candidate::new(0, "A", Some("DEM"), None),
            Candidate::new(1, "A", Some("REP"), None),
            Candidate::new(2, "C", None, None),
        ];
        let all = VotingBlock::builder(preferences::Mallows::new(vec![0, 1, 2], 0.0), 100)
            .add_tactic(tactics::Identity, 0.8)
            .build();
        Election::new((), candidate_pool, [all], methods::Plurality)
    }

    group.bench_function("single election", |b| {
        b.iter(|| {
            let election = election_setup();
            let _outcome = election.run_once(0);
        })
    });

    group.bench_function("multiple elections", |b| {
        b.iter(|| {
            let election = election_setup();
            let _outcomes = election.run_many(100, 0);
        })
    });
}

criterion_group! {
    name = election;
    config = Criterion::default();
    targets = election_benchmarks
}
criterion_main!(election);
