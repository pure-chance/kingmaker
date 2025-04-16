use crate::core::{Ballot, Candidate, Outcome, Profile};

/// A method of tabulating votes and determining the winner of an election.
///
/// A method is defined to be a set of rules (an algorithm) that determines the outcome of an election.
///
/// From a social choice perspective, a method is a social welfare function that ranks potential outcomes by desirability by aggregating collective preferences (a profile).
pub trait Method: Send + Sync + Clone + std::fmt::Debug {
    /// The type of ballot used by this method.
    type Ballot: Ballot;
    /// The type of outcome produced by this method (Single-Winner or Multi-Winner).
    type Winner: Outcome;
    /// Determines the outcome of an election.
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner;
}
