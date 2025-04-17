use kingmaker::prelude::*;

fn main() {
    // configure election
    let candidates = [
        Candidate::new(0, "A", Some("DEM"), Default::default()),
        Candidate::new(1, "B", Some("REP"), Default::default()),
        Candidate::new(2, "C", None, Default::default()),
    ];
    let voting_blocks = [
        VotingBloc::builder(preferences::Mallows::new(vec![0, 1, 2], 1.4), 40)
            .add_tactic(tactics::Identity, 0.8)
            .build(),
        VotingBloc::builder(preferences::Mallows::new(vec![1, 2, 0], 1.0), 45)
            .add_tactic(tactics::Identity, 0.8)
            .build(),
        VotingBloc::builder(preferences::Mallows::new(vec![2, 0, 1], 1.2), 15)
            .add_tactic(tactics::Identity, 0.8)
            .build(),
    ];
    let election = Election::new(candidates, voting_blocks, methods::Plurality);
    // run election
    let outcomes = election.run_many(1_000, 0);
    election.display(&outcomes);
}
