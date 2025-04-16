use kingmaker::prelude::*;

fn main() {
    // configure election(s)
    let candidates = vec![
        Candidate::new(0, "A", Some("DEM"), None),
        Candidate::new(1, "A", Some("REP"), None),
        Candidate::new(2, "C", None, None),
        Candidate::new(3, "D", None, None),
        Candidate::new(4, "E", None, None),
    ];
    let voting_blocs = [
        VotingBloc::builder(preferences::Mallows::new(vec![0, 1, 2, 3, 4], 0.2), 5_000)
            .add_tactic(tactics::Identity, 0.8)
            .add_tactic(tactics::Burial(vec![1]), 0.2)
            .build(),
        VotingBloc::builder(preferences::Mallows::new(vec![2, 1, 4, 3, 0], 0.15), 5_000)
            .add_tactic(tactics::Identity, 0.7)
            .add_tactic(tactics::Burial(vec![1]), 0.3)
            .build(),
    ];
    let election = Election::new(candidates, voting_blocs, methods::Plurality);
    // run election(s)
    let outcomes = election.run_once(0);
    // display outcome
    election.display(&[outcomes]);
}
