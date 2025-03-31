use kingmaker::prelude::*;

fn main() {
    // configure election
    let election = Election::builder((), methods::Plurality)
        .add_candidate(0, "A", Some("DEM"), None)
        .add_candidate(1, "B", Some("REP"), None)
        .add_candidate(2, "C", None, None)
        .add_voting_block(
            preferences::Mallows::new(vec![0, 1, 2], 0.2),
            Strategy::new()
                .add_tactic(tactics::Identity, 0.8)
                .add_tactic(tactics::Burial(vec![1]), 0.2),
            5_000,
        )
        .add_voting_block(
            preferences::Mallows::new(vec![2, 1, 0], 0.15),
            Strategy::new()
                .add_tactic(tactics::Identity, 0.7)
                .add_tactic(tactics::Burial(vec![1]), 0.3),
            5_000,
        )
        .build();
    // run election(s)
    let outcomes = election.run_many(1_000, 0);
    // display outcome
    election.display(outcomes);
}

