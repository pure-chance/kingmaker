// modules
mod impartial;
mod manual;
mod plackett_luce;

// re-exports
pub use impartial::Impartial;
pub use manual::Manual;
pub use plackett_luce::PlackettLuce;

#[cfg(test)]
mod tests {
    use crate::prelude::{
        preferences::{Impartial, Manual, PlackettLuce},
        *,
    };
    use rand::{rngs::StdRng, SeedableRng};

    fn candidate_pool() -> Vec<Candidate> {
        vec![
            Candidate::new(0, "A", Some("DEM"), None),
            Candidate::new(1, "B", Some("REP"), None),
            Candidate::new(2, "C", None, None),
        ]
    }

    #[test]
    fn impartial_outcome() {
        let candidate_pool = candidate_pool();
        let impartial = Impartial;
        let mut rng = StdRng::seed_from_u64(0);
        let outcome: Profile<Ordinal> = impartial.sample(&candidate_pool, 100, &mut rng);
        assert_eq!(outcome.len(), 100);
    }

    #[test]
    fn manual_outcome() {
        let candidate_pool = candidate_pool();
        let profile: Profile<Ordinal> = vec![
            Ordinal(vec![0, 1, 2]),
            Ordinal(vec![1, 2, 0]),
            Ordinal(vec![2, 0, 1]),
            Ordinal(vec![0, 2, 1]),
            Ordinal(vec![1, 0, 2]),
        ]
        .into();
        let manual = Manual::new(profile);
        let mut rng = StdRng::seed_from_u64(0);
        let outcome: Profile<Ordinal> = manual.sample(&candidate_pool, 100, &mut rng);
        assert_eq!(outcome.len(), 100);
    }

    #[test]
    fn plackett_luce_outcome() {
        let candidate_pool = candidate_pool();
        let weights = vec![(0, 1.0), (1, 1.1), (2, 1.0)];
        let plackett_luce = PlackettLuce::new(weights);
        let mut rng = StdRng::seed_from_u64(0);
        let outcome: Profile<Ordinal> = plackett_luce.sample(&candidate_pool, 100, &mut rng);
        assert_eq!(outcome.len(), 100);
    }
}
