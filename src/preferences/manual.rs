use crate::core::*;
use rand::{rngs::StdRng, Rng};

/// A special preference type where preferences are drawn randomly from real-world data.
#[derive(Debug, Clone)]
pub struct Manual<B: Ballot> {
    votes: Profile<B>,
}

impl<B: Ballot> Manual<B> {
    pub fn new(profile: Profile<B>) -> Self {
        Manual { votes: profile }
    }
}

impl<B: Ballot> Preference<B> for Manual<B> {
    fn draw(&self, _candidate_pool: &[Candidate], rng: &mut StdRng) -> B {
        self.votes[rng.random_range(0..self.votes.len())].to_owned()
    }
}
