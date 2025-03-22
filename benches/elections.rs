use criterion::{criterion_group, criterion_main, Criterion};
use kingmaker::prelude::*;

pub fn election_benchmarks(c: &mut Criterion) {
    let mut _group = c.benchmark_group("election");

    fn _election_setup() -> Election<Ordinal, (), methods::Plurality> {
        Election::builder((), methods::Plurality)
            .add_candidate(0, "A", Some("DEM"), None)
            .add_candidate(1, "B", Some("REP"), None)
            .add_candidate(2, "C", None, None)
            .add_voting_block(
                preferences::Mallows::new(vec![0, 1, 2], 0.0),
                Strategy::new().add_tactic(tactics::Identity, 1.0),
                100,
            )
            .build()
    }
}

criterion_group! {
    name = election;
    config = Criterion::default();
    targets = election_benchmarks
}
criterion_main!(election);
