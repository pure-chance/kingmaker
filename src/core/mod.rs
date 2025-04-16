//! The core components of Kingmaker.
mod ballot;
mod candidate;
mod election;
mod method;
mod outcome;
mod preference;
mod tactic;
mod voting_bloc;

pub use ballot::{Ballot, Cardinal, Nominal, Ordinal, Profile};
pub use candidate::Candidate;
pub use election::Election;
pub use method::Method;
pub use outcome::{MultiWinner, Outcome, SingleWinner};
pub use preference::Preference;
pub use tactic::Tactic;
pub use voting_bloc::VotingBloc;

/// A unique identifier for a candidate
pub(crate) type Id = u16;
