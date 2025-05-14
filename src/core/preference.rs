use std::fmt::Debug;

use rand::rngs::StdRng;

use crate::core::{Ballot, Candidate, Profile};

/// A preference can be conceptualized as the preferences of the voter as they exist in their head before being written down.
///
/// It is defined as a distribution over possible realizations (ballots), where at election time, one such realization is drawn.
pub trait Preference<B: Ballot>: Send + Sync + Debug {
    /// Draws a ballot from the preference distribution.
    fn draw(&self, candidates: &[Candidate], rng: &mut StdRng) -> B;
    /// Samples a profile from the preference distribution.
    #[inline]
    fn sample(&self, candidates: &[Candidate], sample_size: usize, rng: &mut StdRng) -> Profile<B> {
        Profile::new((0..sample_size).map(|_| self.draw(candidates, rng)))
    }
}
