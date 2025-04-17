use crate::core::{Id, Ordinal, Tactic};

/// The compromise tactic places candidates that are more likely to win in ahead of the true voters preferences. The `Vec<Id>` specifies the electable candidates (in order), with the new ranking being `compromise + (ballot - compromise)`.
#[derive(Debug)]
pub struct Compromise {
    compromises: Vec<Id>,
}

impl Compromise {
    #[must_use]
    pub const fn new(compromises: Vec<Id>) -> Self {
        Self { compromises }
    }
}

impl Tactic<Ordinal> for Compromise {
    fn apply(&self, ballot: Ordinal) -> Ordinal {
        let mut compromised_ranking: Vec<Id> = (*ballot).clone();
        compromised_ranking.retain(|c| !self.compromises.contains(c));
        let mut ranking = self.compromises.clone();
        ranking.append(&mut compromised_ranking);
        Ordinal(ranking)
    }
}
