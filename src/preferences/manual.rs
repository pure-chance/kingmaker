use crate::core::{Ballot, Candidate, Preference, Profile};

use rand::{Rng, rngs::StdRng};
use serde::Serialize;

/// Manual Preference Model
///
/// The `Manual` preference type represents a special case where preferences are drawn randomly from real-world data instead of being generated through a probabilistic model like Mallows or Plackett-Luce.
///
/// # Preference Selection
///
/// Given a dataset of real-world ballots, a preference ranking `(c_1, c_2, ..., c_n)` is sampled randomly from the dataset. Formally, if the dataset contains `M` unique rankings, the probability of selecting a specific ranking is:
///
/// ```math
/// P(c_1, c_2, ..., c_n) = 1 / M
/// ```
///
/// where:
/// - `M` is the total number of unique rankings in the dataset.
/// - Each ranking is assumed to be equally probable unless additional weighting is applied.
///
/// # Interpretation
///
/// - This method ensures that generated preferences are grounded in empirical data.
/// - Unlike Mallows or Plackett-Luce, `Manual` does not impose a probabilistic structure, making it suitable for scenarios where real-world rankings are preferred.
/// - If the dataset is small, rankings may be repeated frequently in generated samples.
#[derive(Debug, Clone, Serialize)]
pub struct Manual<B: Ballot> {
    votes: Profile<B>,
}

impl<B: Ballot> Manual<B> {
    /// Instantiates a new Manual preference model.
    #[must_use]
    pub const fn new(profile: Profile<B>) -> Self {
        Self { votes: profile }
    }
}

impl<B: Ballot> Preference<B> for Manual<B> {
    #[inline]
    fn draw(&self, _candidates: &[Candidate], rng: &mut StdRng) -> B {
        self.votes[rng.random_range(0..self.votes.len())].to_owned()
    }
}
