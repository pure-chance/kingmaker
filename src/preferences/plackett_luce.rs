use crate::core::{Candidate, Id, Ordinal, Preference};
use rand::{distr::weighted::WeightedIndex, prelude::*};

/// ...TODO
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
