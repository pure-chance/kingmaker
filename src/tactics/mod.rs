// modules
mod identity;

// re-exports
pub use identity::Identity;

#[cfg(test)]
mod tests {
    use crate::prelude::{tactics::Identity, *};
    use std::collections::BTreeSet;

    #[test]
    fn test_identity() {
        let identity = Identity;
        let nominal = Nominal(BTreeSet::new());
        assert_eq!(identity.apply(nominal.clone()), nominal);
    }
}
