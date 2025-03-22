//! The core components of Kingmaker.
mod ballot;
mod candidate;
mod election;
mod method;
mod outcome;
mod preference;
mod tactic;
mod voter;

pub use ballot::{Ballot, Cardinal, Nominal, Ordinal, Profile};
pub use candidate::Candidate;
pub use election::Election;
pub use method::Method;
pub use outcome::{MultiWinner, Outcome, SingleWinner};
pub use preference::Preference;
pub use tactic::{Strategy, Tactic};
pub use voter::VotingBlock;

/// A unique identifier for a candidate
pub type Id = u16;
