use crate::core::*;

use rand::{Rng, rngs::StdRng};
use serde::Serialize;

/// The `Impartial` preference model generates rankings by assuming that all possible rankings are equally likely, leading to a uniform random distribution over all permutations.
///
/// # Probability Distribution
///
/// Given `n` candidates, each possible ranking `(c_1, c_2, ..., c_n)` is selected with equal probability:
///
/// ```text
/// P(c_1, c_2, ..., c_n) = 1 / n!
/// ```
///
/// where:
/// - `n!` is the total number of possible rankings (permutations of `n` candidates).
/// - Every ranking is equally likely, ensuring no inherent bias.
///
/// # Interpretation
///
/// - The `Impartial` model represents a completely neutral preference structure.
/// - It is often used as a baseline for analyzing voting systems.
/// - As `n` increases, the number of possible rankings grows factorially, making exhaustive enumeration infeasible for large values of `n`.
#[derive(Debug, Clone, Serialize)]
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
