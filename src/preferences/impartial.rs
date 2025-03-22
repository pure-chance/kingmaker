use crate::core::*;
use rand::{rngs::StdRng, Rng};

/// A preference that is uniform across all realizations. There is an equal likelihood of choosing any of the possible ballots.
#[derive(Debug, Clone)]
pub struct Impartial;

impl Impartial {
    pub fn new() -> Self {
        Impartial
    }
}

impl Default for Impartial {
    fn default() -> Self {
        Self::new()
    }
}

impl Preference<Cardinal> for Impartial {
    fn draw(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Cardinal {
        const RANGE: usize = 5;
        Cardinal(
            candidate_pool
                .iter()
                .map(|candidate| (candidate.id() as Id, rng.random_range(0..=RANGE)))
                .collect(),
        )
    }
}
impl Preference<Ordinal> for Impartial {
    fn draw(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Ordinal {
        Ordinal({
            let mut points: Vec<(Id, f32)> = candidate_pool
                .iter()
                .map(|candidate| (candidate.id() as Id, rng.random_range(0.0..1.0)))
                .collect();
            points.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            points.iter().map(|(id, _)| *id).collect()
        })
    }
}

impl Preference<Nominal> for Impartial {
    fn draw(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Nominal {
        Nominal(
            candidate_pool
                .iter()
                .filter(|_| rng.random_bool(0.5))
                .map(|candidate| candidate.id())
                .collect(),
        )
    }
}
