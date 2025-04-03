use std::fmt::Debug;

use crate::core::Ballot;

/// A tactic is a method of altering one's ballot to maximize (or at least increase) social welfare.
///
/// Note that this implementation considers tactics to be a separate process that occurs *after* realization. This is a limitation of the model.
pub trait Tactic<B: Ballot>: Send + Sync + Debug {
    fn apply(&self, ballot: B) -> B;
}
