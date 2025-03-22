use crate::core::*;
use rand::{distr::weighted::WeightedIndex, prelude::*};

/// The Plackett-Luce model is a probabilistic model for generating preference rankings based on candidates' weights. A ballot is constructed by incrementally selecting candidates, where the probability of choosing a candidate is proportional to its weight.
///
/// # Probability Distribution
///
/// Given a set of candidates with associated weights `w_1, w_2, ..., w_n`, the probability of selecting a ranking `(c_1, c_2, ..., c_n)` is:
///
/// ```math
/// P(c_1, c_2, ..., c_n) =
/// (w_{c_1} / W) *
/// (w_{c_2} / (W - w_{c_1})) * ...
/// (w_{c_n} / (W - Σ_{i=1}^{n-1} w_{c_i}))
/// ```
///
/// where:
/// - `w_{c_i}` is the weight of candidate `c_i`.
/// - `W` is the sum of all candidate weights.
/// - At each step, a candidate is selected with probability proportional to its weight among the remaining candidates.
///
/// # Interpretation of Weights
///
/// - Higher weights correspond to higher chances of being ranked earlier.
/// - The model naturally handles different numbers of candidates and allows flexible weight assignments.
/// - If all candidates have equal weights, the ranking is uniformly random.
#[derive(Debug, Clone)]
pub struct PlackettLuce {
    weights: Vec<(Id, f32)>,
}

impl PlackettLuce {
    pub fn new(weights: Vec<(Id, f32)>) -> Self {
        Self { weights }
    }
}

impl Preference<Ordinal> for PlackettLuce {
    fn draw(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Ordinal {
        let mut weights = self.weights.clone();
        let mut ballot = Vec::with_capacity(candidate_pool.len());
        while !weights.is_empty() {
            let sampled_id = WeightedIndex::new(weights.iter().map(|(_, w)| w))
                .unwrap()
                .sample(rng);
            ballot.push(weights[sampled_id].0);
            let _ = weights.remove(sampled_id);
        }
        Ordinal(ballot)
    }
}
