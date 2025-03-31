//! A collection of `tactics` that can be used to represent the tactics (and thus strategies) of voters in an election.
mod burial;
mod compromise;
mod identity;
mod pushover;

pub use burial::Burial;
pub use compromise::Compromise;
pub use identity::Identity;
pub use pushover::Pushover;

#[cfg(test)]
mod tests {
    use crate::prelude::{
        tactics::{Burial, Compromise, Identity, Pushover},
        *,
    };
    use std::collections::BTreeSet;

    #[test]
    fn identity_tactic() {
        let identity = Identity;
        let nominal = Nominal(BTreeSet::new());
        assert_eq!(identity.apply(nominal.clone()), nominal);
    }

    #[test]
    fn compromise_tactic() {
        let compromise = Compromise(vec![2]);
        let ordinal = Ordinal(vec![0, 2, 1]);
        assert_eq!(compromise.apply(ordinal), Ordinal(vec![2, 0, 1]))
    }

    #[test]
    fn burial_tactic() {
        let burial = Burial(vec![2]);
        let ordinal = Ordinal(vec![0, 2, 1]);
        assert_eq!(burial.apply(ordinal), Ordinal(vec![0, 1, 2]))
    }

    #[test]
    fn pushover_tactic() {
        let pushover = Pushover(vec![0], vec![2]);
        let ordinal = Ordinal(vec![0, 1, 2]);
        assert_eq!(pushover.apply(ordinal), Ordinal(vec![0, 2, 1]))
    }
}
