use std::sync::Arc;

use super::{Ballot, Preference, Strategy};

/// A block of voters, such as democrats / republicans, or rural / suburban / urban.
///
/// A block of voters is considered to have a single aggregate preference and set of tactics. They represent the sum total distribution across all the voters in the block. When a voter draws from this distribution, that is the expression of their preferences / individuality.
#[derive(Debug)]
pub struct VotingBlock<B: Ballot> {
    preference: Arc<dyn Preference<B>>,
    strategy: Strategy<B>,
    members: usize,
}

impl<B: Ballot> VotingBlock<B> {
    /// Create a new voting block
    pub fn new(
        preference: impl Preference<B> + 'static,
        strategy: Strategy<B>,
        members: usize,
    ) -> Self {
        Self {
            preference: Arc::new(preference),
            strategy,
            members,
        }
    }
    /// Get the preferences of the voting block
    pub fn preferences(&self) -> Arc<dyn Preference<B>> {
        self.preference.clone()
    }
    /// Get the preferences of the voting block
    pub fn strategy(&self) -> Strategy<B> {
        self.strategy.clone()
    }
    /// Get the # of members in the voting block
    pub fn members(&self) -> usize {
        self.members
    }
}
