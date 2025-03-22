// modules
mod plurality;
mod random_dictator;

// re-exports
pub use plurality::Plurality;
pub use random_dictator::RandomDictator;

#[cfg(test)]
mod tests {
    use crate::prelude::{
        methods::{Plurality, RandomDictator},
        *,
    };
    use std::collections::{BTreeMap, BTreeSet};

    fn candidate_pool() -> Vec<Candidate> {
        vec![
            Candidate::new(0, "A", Some("DEM"), None),
            Candidate::new(1, "B", Some("REP"), None),
            Candidate::new(2, "C", None, None),
        ]
    }

    fn nominal_ballots() -> Profile<Nominal> {
        vec![
            Nominal(BTreeSet::from([])),
            Nominal(BTreeSet::from([])),
            Nominal(BTreeSet::from([0, 1])),
            Nominal(BTreeSet::from([1])),
            Nominal(BTreeSet::from([2])),
        ]
        .into()
    }

    fn ordinal_ballots() -> Profile<Ordinal> {
        vec![
            Ordinal(vec![0, 1, 2]),
            Ordinal(vec![1, 0, 2]),
            Ordinal(vec![2, 1, 0]),
            Ordinal(vec![2, 0, 1]),
            Ordinal(vec![0, 1, 2]),
        ]
        .into()
    }

    fn cardinal_ballots() -> Profile<Cardinal> {
        vec![
            Cardinal(BTreeMap::from([(0, 4), (1, 3), (2, 4)])),
            Cardinal(BTreeMap::from([(0, 4), (1, 1), (2, 2)])),
            Cardinal(BTreeMap::from([(0, 0), (1, 5), (2, 0)])),
            Cardinal(BTreeMap::from([(0, 0), (1, 2), (2, 1)])),
            Cardinal(BTreeMap::from([(0, 4), (1, 3), (2, 0)])),
        ]
        .into()
    }

    #[test]
    fn random_dictator_outcome() {
        let candidate_pool = candidate_pool();
        let ballots: Profile<Ordinal> = ordinal_ballots();
        let outcome = RandomDictator.outcome(&candidate_pool, &ballots);
        assert_eq!(outcome, SingleWinner::win(&candidate_pool, 0));
    }

    #[test]
    fn plurality_outcome() {
        let candidate_pool = candidate_pool();
        let ballots = ordinal_ballots();
        let outcome = Plurality.outcome(&candidate_pool, &ballots);
        assert_eq!(outcome, SingleWinner::tie(&candidate_pool, &[0, 2]));
    }
}
