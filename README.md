# Kingmaker

Kingmaker is a modular, performant, social choice framework for the simulation, computation, and analysis of strategic voting. It has a focus on the study of strategic or tactical voting, which is concerned with questions such as:

1. Which strategies are most effective (for a given voting method)?
2. To what degree do strategies impact the outcome of elections?
3. Which voting methods minimize strategic voting?

and much more.

## Installation

Kingmaker is currently in pre-release and not yet available on `crates.io`. You can build from source using `cargo`:

```sh
git clone https://github.com/Approximately-Equal/kingmaker.git
cd kingmaker
cargo run --release
```

## Getting Started

To get started with Kingmaker, just configure the election parameters and run the simulation. Here's a (somewhat) minimal example:

```rust
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
        VotingBloc::builder(
            preferences::Mallows::new(vec![0, 1, 2, 3, 4], 0.2),
            5_000,
        )
        .add_tactic(tactics::Identity, 0.8)
        .add_tactic(tactics::Burial(vec![1]), 0.2)
        .build(),
        VotingBloc::builder(
            preferences::Mallows::new(vec![2, 1, 4, 3, 0], 0.15),
            5_000,
        )
        .add_tactic(tactics::Identity, 0.7)
        .add_tactic(tactics::Burial(vec![1]), 0.3)
        .build(),
    ];
    let election =
        Election::new((), candidates, voting_blocs, methods::Plurality);
    // run election(s)
    let outcomes = election.run_once(0);
    // display outcome
    election.display([outcomes]);
}
```

## Acknowledgments

- Much thanks to the folks at [pref_voting](https://github.com/voting-tools/pref_voting) for their comprehensive work on social choice research.
- I'd also like to thank my advisor Michael Pearce for his guidance and support.
