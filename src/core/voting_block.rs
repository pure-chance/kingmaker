use std::sync::Arc;

use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

use crate::core::{Ballot, Candidate, Preference, Tactic};
use crate::tactics::Identity;

use super::Profile;

/// A block of voters, such as democrats / republicans, or rural / suburban / urban.
///
/// A block of voters is considered to have a single aggregate preference and set of tactics. They represent the sum total distribution across all the voters in the block. When a voter draws from this distribution, that is the expression of their preferences / individuality.
#[derive(Debug)]
pub struct VotingBlock<B: Ballot> {
    preference: Arc<dyn Preference<B>>,
    strategy: Vec<(Arc<dyn Tactic<B>>, f32)>,
    members: usize,
}

impl<B: Ballot> VotingBlock<B> {
    /// Build a new voting block with the builder
    pub fn builder(
        preference: impl Preference<B> + 'static,
        members: usize,
    ) -> VotingBlockBuilder<B> {
        VotingBlockBuilder::new(preference, members)
    }
    /// Create a new voting block
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
    /// Get the preferences of the voting block
    pub fn preferences(&self) -> Arc<dyn Preference<B>> {
        self.preference.clone()
    }
    /// Get the preferences of the voting block
    pub fn strategy(&self) -> &[(Arc<dyn Tactic<B>>, f32)] {
        &self.strategy
    }
    /// Get the # of members in the voting block
    pub fn members(&self) -> usize {
        self.members
    }
    pub fn realize(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Profile<B> {
        (0..self.members())
            .map(|_| self.preferences().draw(candidate_pool, rng))
            .collect()
    }
    pub fn vote(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Profile<B> {
        let weights: Vec<f32> = self.strategy.iter().map(|(_, weight)| *weight).collect();
        let dist = WeightedIndex::new(&weights).expect("Weights should be valid");
        (0..self.members())
            .map(|_| {
                let honest_ballot = self.preferences().draw(candidate_pool, rng);
                let tactic = &self.strategy[dist.sample(rng)].0;
                tactic.apply(honest_ballot)
            })
            .collect()
    }
}

pub struct VotingBlockBuilder<B: Ballot> {
    preference: Arc<dyn Preference<B>>,
    strategy: Vec<(Arc<dyn Tactic<B>>, f32)>,
    members: usize,
}

impl<B: Ballot> VotingBlockBuilder<B> {
    /// Create a new voting block builder
    pub fn new(preference: impl Preference<B> + 'static, members: usize) -> Self {
        Self {
            preference: Arc::new(preference),
            strategy: Vec::new(),
            members,
        }
    }
    /// Add a tactic to the voting block
    pub fn add_tactic(mut self, tactic: impl Tactic<B> + 'static, weight: f32) -> Self {
        self.strategy.push((Arc::new(tactic), weight));
        self
    }
    /// Build the voting block
    pub fn build(mut self) -> VotingBlock<B> {
        if self.strategy.is_empty() {
            self.strategy.push((Arc::new(Identity), 1.0f32));
        }
        VotingBlock::new(self.preference, self.strategy, self.members)
    }
}
