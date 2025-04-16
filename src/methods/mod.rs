//! A collection of `methods` that can be used to tabulate the results of an election.
mod approval;
mod borda;
mod instant_runoff;
mod plurality;
mod random_dictator;
mod single_transferable_vote;
mod star;

pub use approval::Approval;
pub use borda::Borda;
pub use instant_runoff::IRV;
pub use plurality::Plurality;
pub use random_dictator::RandomDictator;
pub use single_transferable_vote::STV;
pub use star::Star;

#[cfg(test)]
mod tests {
    use crate::prelude::{methods::*, *};
    use std::collections::{BTreeMap, BTreeSet};

    fn candidates() -> Vec<Candidate> {
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
            Ordinal(vec![2, 1, 0]),
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
        let candidates = candidates();
        let ballots: Profile<Ordinal> = ordinal_ballots();
        let outcome = RandomDictator.outcome(&candidates, ballots);
        assert_eq!(outcome, SingleWinner::win(&candidates, 0));
    }

    #[test]
    fn plurality_outcome() {
        let candidates = candidates();
        let ballots = ordinal_ballots();
        let outcome = Plurality.outcome(&candidates, ballots);
        assert_eq!(outcome, SingleWinner::win(&candidates, 2));
    }

    #[test]
    fn approval_outcome() {
        let candidates = candidates();
        let ballots = nominal_ballots();
        let outcome = Approval.outcome(&candidates, ballots);
        assert_eq!(outcome, SingleWinner::win(&candidates, 1));
    }

    #[test]
    fn borda_outcome() {
        let candidates = candidates();
        let ballots = ordinal_ballots();
        let outcome = Borda.outcome(&candidates, ballots);
        assert_eq!(outcome, SingleWinner::tie(&candidates, &[0, 1, 2]));
    }

    #[test]
    fn star_outcome() {
        let candidates = candidates();
        let ballots = cardinal_ballots();
        let outcome = Star.outcome(&candidates, ballots);
        assert_eq!(outcome, SingleWinner::win(&candidates, 0));
    }

    #[test]
    fn instant_runoff_outcome() {
        let candidates = candidates();
        let ballots = ordinal_ballots();
        let outcome = IRV.outcome(&candidates, ballots);
        assert_eq!(outcome, SingleWinner::tie(&candidates, &[0, 2]));
    }

    #[test]
    fn single_transferable_vote_outcome() {
        let candidates = candidates();
        let ballots = ordinal_ballots();
        let outcome = STV::new(2).outcome(&candidates, ballots);
        assert_eq!(outcome, MultiWinner::seats(&candidates, &[1, 2]));
    }
}
