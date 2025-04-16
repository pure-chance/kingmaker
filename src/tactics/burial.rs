use crate::core::{Id, Ordinal, Tactic};

/// The burial tactic buries some leading candidate to hopefully knock them out early in the methods. The `Vec<Id>` specifies the candidates to bury, with the new ranking being `(ballot - buried) + buried`.
#[derive(Debug)]
pub struct Burial(pub Vec<Id>);

impl Tactic<Ordinal> for Burial {
    fn apply(&self, ballot: Ordinal) -> Ordinal {
        let mut ranking: Vec<Id> = (*ballot).clone();
        ranking.retain(|c| !self.0.contains(c));
        ranking.append(&mut self.0.clone());
        Ordinal(ranking)
    }
}
