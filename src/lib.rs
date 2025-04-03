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
//! To get started with Kingmaker, just configure the election parameters and run the simulation. Here's a minimal example:
//!
//! ```rust
//! use kingmaker::prelude::*;
//!
//! // configure election(s)
//! let candidate_pool = vec![
//!     Candidate::new(0, "A", Some("DEM"), None),
//!     Candidate::new(1, "A", Some("REP"), None),
//!     Candidate::new(2, "C", None, None),
//! ];
//! let voter_pool = [
//!     VotingBlock::builder(preferences::Mallows::new(vec![0, 1, 2], 0.2), 5_000)
//!         .add_tactic(tactics::Identity, 0.8)
//!         .add_tactic(tactics::Burial(vec![1]), 0.2)
//!         .build(),
//!     VotingBlock::builder(preferences::Mallows::new(vec![2, 1, 0], 0.15), 5_000)
//!         .add_tactic(tactics::Identity, 0.7)
//!         .add_tactic(tactics::Burial(vec![1]), 0.3)
//!         .build(),
//! ];
//! let election = Election::new((), candidate_pool, voter_pool, methods::Plurality);
//! // run election(s)
//! let outcomes = election.run_many(1_000, 0);
//! // display outcome
//! election.display(outcomes);
//! ```
pub mod core;
pub mod methods;
pub mod preferences;
pub mod tactics;

pub mod prelude {
    pub use crate::core::*;
    pub use crate::methods;
    pub use crate::preferences;
    pub use crate::tactics;
}
