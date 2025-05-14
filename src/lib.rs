#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

//! # Kingmaker
//!
//! Kingmaker is a modular, performant, social choice framework for the simulation, computation, and analysis of strategic voting. It has a focus on the study of strategic or tactical voting, which is concerned with questions such as:
//!
//! 1. Which strategies are most effective (for a given voting method)?
//! 2. To what degree do strategies impact the outcome of elections?
//! 3. Which voting methods minimize strategic voting?
//!
//! and much more.
//!
//! ## Getting Started
//!
//! To get started with Kingmaker, just configure the election parameters and run the simulation. Here's an example:
//!
//! ```rust
//! use kingmaker::prelude::*;
//!
//! fn main() {
//!     // configure election
//!     let candidates = [
//!         Candidate::new(0, "Alice", Some("DEM"), Default::default()),
//!         Candidate::new(1, "Bliar", Some("REP"), Default::default()),
//!         Candidate::new(2, "Cybil", None, Default::default()),
//!     ];
//!     let voting_blocks = [
//!         VotingBloc::builder(preferences::Mallows::new(vec![0, 2, 1], 1.4), 40)
//!             .add_tactic(tactics::Burial::new(vec![1]), 0.8) // Do not vote for the opposition!
//!             .add_tactic(tactics::Identity, 0.2)
//!             .build(),
//!         VotingBloc::builder(preferences::Mallows::new(vec![1, 2, 0], 1.0), 45)
//!             .add_tactic(tactics::Burial::new(vec![0]), 0.8) // Do not vote for the opposition!
//!             .add_tactic(tactics::Identity, 0.2)
//!             .build(),
//!         VotingBloc::builder(preferences::Mallows::new(vec![2, 0, 1], 1.2), 15)
//!             .add_tactic(tactics::Identity, 0.8)
//!             .add_tactic(tactics::Compromise::new(vec![0]), 0.2) // Compromise for A (DEM) instead
//!             .build(),
//!     ];
//!     let election = Election::new(candidates, voting_blocks, methods::IRV).unwrap();
//!     // run election
//!     let outcomes = election.run_many(1_000, 0);
//!     election.display(&outcomes);
//! }
//! ```
pub mod core;
pub mod methods;
pub mod preferences;
pub mod tactics;

/// The kingmaker standard library. This includes all the core components and common preferences, tactics, and methods that tend to be used in real-world elections.
pub mod prelude {
    pub use crate::core::*;
    pub use crate::methods;
    pub use crate::preferences;
    pub use crate::tactics;
}
