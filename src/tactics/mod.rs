// modules
mod compromise;
mod identity;

// re-exports
pub use compromise::Compromise;
pub use identity::Identity;

#[cfg(test)]
mod tests {
    use crate::prelude::{
        tactics::{Compromise, Identity},
        *,
    };
    use std::collections::BTreeSet;

    #[test]
    fn test_identity() {
        let identity = Identity;
        let nominal = Nominal(BTreeSet::new());
        assert_eq!(identity.apply(nominal.clone()), nominal);
    }

    #[test]
    fn test_compromise() {
        let compromise = Compromise(vec![2]);
        let ordinal = Ordinal(vec![0, 2, 1]);
        assert_eq!(compromise.apply(ordinal), Ordinal(vec![2, 0, 1]))
    }
}
