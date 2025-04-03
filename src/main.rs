use kingmaker::prelude::*;

fn main() {
    // configure election(s)
    let candidate_pool = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "A", Some("REP"), None),
        Candidate::new(2, "C", None, None),
    ];
    let voter_pool = [
        VotingBlock::builder(preferences::Mallows::new(vec![0, 1, 2], 0.2), 5_000)
            .add_tactic(tactics::Identity, 0.8)
            .add_tactic(tactics::Burial(vec![1]), 0.2)
            .build(),
        VotingBlock::builder(preferences::Mallows::new(vec![2, 1, 0], 0.15), 5_000)
            .add_tactic(tactics::Identity, 0.7)
            .add_tactic(tactics::Burial(vec![1]), 0.3)
            .build(),
    ];
    let election = Election::new((), candidate_pool, voter_pool, methods::Plurality);
    // run election(s)
    let outcomes = election.run_many(1_000, 0);
    // display outcome
    println!("{:#?}", election.write(outcomes));
}
