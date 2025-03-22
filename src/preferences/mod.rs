// modules
mod impartial;

// re-exports
pub use impartial::Impartial;

#[cfg(test)]
mod tests {
    use crate::prelude::{preferences::Impartial, *};
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
}
