use std::sync::Arc;

use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

use crate::core::{Ballot, Candidate, Preference, Tactic};
use crate::tactics::Identity;

use crate::core::Profile;

/// A bloc of voters, e.g. democrats / republicans or rural / suburban / urban.
///
/// A bloc of voters is considered to have a single aggregate preference and set of tactics. They represent the sum total distribution across all the voters in the bloc. When a voter draws from this distribution, that is the expression of their preferences / individuality.
#[derive(Debug)]
pub struct VotingBloc<B: Ballot> {
    /// The preference of the voting bloc
    preference: Arc<dyn Preference<B>>,
    /// The strategy of the voting bloc
    strategy: Vec<(Arc<dyn Tactic<B>>, f32)>,
    /// The number of members in the voting bloc
    members: usize,
}

impl<B: Ballot> VotingBloc<B> {
    /// Build a new voting bloc with the builder
    pub fn builder(
        preference: impl Preference<B> + 'static,
        members: usize,
    ) -> VotingBlocBuilder<B> {
        VotingBlocBuilder::new(preference, members)
    }
    /// Create a new voting bloc
    fn new(
        preference: Arc<dyn Preference<B>>,
        strategy: Vec<(Arc<dyn Tactic<B>>, f32)>,
        members: usize,
    ) -> Self {
        Self {
            preference,
            strategy,
            members,
        }
    }
    /// Get the preferences of the voting bloc
    pub fn preferences(&self) -> Arc<dyn Preference<B>> {
        self.preference.clone()
    }
    /// Get the preferences of the voting bloc
    pub fn strategy(&self) -> &[(Arc<dyn Tactic<B>>, f32)] {
        &self.strategy
    }
    /// Get the # of members in the voting bloc
    pub fn members(&self) -> usize {
        self.members
    }
    /// Realize preferences to a profile
    pub fn realize(&self, candidates: &[Candidate], rng: &mut StdRng) -> Profile<B> {
        (0..self.members())
            .map(|_| self.preferences().draw(candidates, rng))
            .collect()
    }
    /// Realize preferences to a profile and apply strategy to them.
    ///
    /// In other words, make the voting bloc vote.
    ///
    /// # Panics
    ///
    /// Panics if the weights are not valid.
    pub fn vote(&self, candidates: &[Candidate], rng: &mut StdRng) -> Profile<B> {
        let weights: Vec<f32> = self.strategy.iter().map(|(_, weight)| *weight).collect();
        let dist = WeightedIndex::new(&weights).expect("Weights should be valid");
        (0..self.members())
            .map(|_| {
                let honest_ballot = self.preferences().draw(candidates, rng);
                let tactic = &self.strategy[dist.sample(rng)].0;
                tactic.apply(honest_ballot)
            })
            .collect()
    }
}

pub struct VotingBlocBuilder<B: Ballot> {
    preference: Arc<dyn Preference<B>>,
    strategy: Vec<(Arc<dyn Tactic<B>>, f32)>,
    members: usize,
}

impl<B: Ballot> VotingBlocBuilder<B> {
    /// Create a new voting bloc builder
    pub fn new(preference: impl Preference<B> + 'static, members: usize) -> Self {
        Self {
            preference: Arc::new(preference),
            strategy: Vec::new(),
            members,
        }
    }
    /// Add a tactic to the voting bloc
    pub fn add_tactic(mut self, tactic: impl Tactic<B> + 'static, weight: f32) -> Self {
        self.strategy.push((Arc::new(tactic), weight));
        self
    }
    /// Build the voting bloc
    pub fn build(mut self) -> VotingBloc<B> {
        if self.strategy.is_empty() {
            self.strategy.push((Arc::new(Identity), 1.0f32));
        }
        VotingBloc::new(self.preference, self.strategy, self.members)
    }
}
