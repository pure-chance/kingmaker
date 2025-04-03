use super::{Ballot, Candidate, Outcome, Profile};

/// A method of tabulating votes and determining the winner of an election.
///
/// A method is defined to be a set of rules (an algorithm) that determines the outcome of an election.
///
/// From a social choice perspective, a method is a social welfare function that ranks potential outcomes by desirability by aggregating collective preferences (a profile).
pub trait Method: Send + Sync {
    type Ballot: Ballot;
    type Winner: Outcome;
    fn outcome(&self, candidate_pool: &[Candidate], profile: Profile<Self::Ballot>)
        -> Self::Winner;
}
