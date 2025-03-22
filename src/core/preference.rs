use super::{Ballot, Candidate, Profile};
use rand::rngs::StdRng;
use std::fmt::Debug;

/// A preference can be conceptualized as the preferences of the voter as they exist in their head before being written down.
///
/// It is defined as a distribution over possible realizations (ballots), where at election time, one such realization is drawn.
pub trait Preference<B: Ballot>: Send + Sync + Debug {
    fn draw(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> B;
    fn sample(
        &self,
        candidate_pool: &[Candidate],
        sample_size: usize,
        rng: &mut StdRng,
    ) -> Profile<B> {
        Profile(
            (0..sample_size)
                .map(|_| self.draw(candidate_pool, rng))
                .collect(),
        )
    }
}
