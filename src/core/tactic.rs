use std::fmt::Debug;
use std::sync::Arc;

use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::rngs::StdRng;

use super::{Ballot, Profile};

/// A strategy is a distribution over tactics.
///
/// When a voter applies a strategy, they will randomly draw a tactic from the distribution (according to the weights), and apply it to their ballot.
#[derive(Debug, Clone)]
pub struct Strategy<B: Ballot> {
    tactics: Vec<(Arc<dyn Tactic<B>>, f32)>,
}

impl<B: Ballot> Strategy<B> {
    pub fn new() -> Self {
        Self {
            tactics: Vec::new(),
        }
    }
    pub fn add_tactic(mut self, tactic: impl Tactic<B> + 'static, probability: f32) -> Self {
        let tactic: Arc<dyn Tactic<B>> = Arc::new(tactic);
        self.tactics.push((tactic, probability));
        self
    }
    pub fn tactics(&self) -> Vec<Arc<dyn Tactic<B>>> {
        self.tactics
            .iter()
            .map(|(tactic, _)| tactic.clone())
            .collect()
    }
    pub fn weights(&self) -> Vec<f32> {
        self.tactics.iter().map(|(_, weight)| *weight).collect()
    }
    pub fn apply(&self, ballot: B, rng: &mut StdRng) -> B {
        let weights = self.weights();
        let tactic = &self.tactics()[WeightedIndex::new(&weights).unwrap().sample(rng)];
        tactic.apply(ballot)
    }
    pub fn apply_profile(&self, profile: &Profile<B>, rng: &mut StdRng) -> Profile<B> {
        profile
            .iter()
            .map(|ballot| self.apply(ballot.clone(), rng))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<B: Ballot> Default for Strategy<B> {
    fn default() -> Self {
        Self::new()
    }
}

/// A tactic is a method of altering one's ballot to maximize (or at least increase) social welfare.
///
/// Note that this implementation considers tactics to be a separate process that occurs *after* realization. This is a limitation of the model.
pub trait Tactic<B: Ballot>: Send + Sync + Debug {
    fn apply(&self, ballot: B) -> B;
}
