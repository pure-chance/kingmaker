use crate::core::{Ballot, Tactic};

/// The identity tactic does nothing and returns the honest ballot.
#[derive(Debug)]
pub struct Identity;

impl<B: Ballot> Tactic<B> for Identity {
    fn apply(&self, ballot: B) -> B {
        ballot
    }
}
