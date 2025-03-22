use crate::prelude::{Id, Ordinal, Tactic};

/// The pushover tactic places pushover candidates highly, not to get them elected, but to hopefully knock out stronger candidates in early rounds of voting before losing to their true preferences. The `Vec<Id>` specifies the pushover candidates (in order), with the new ranking being `preferred + pushover + others`.
#[derive(Debug)]
pub struct Pushover(pub Vec<Id>, pub Vec<Id>);

impl Tactic<Ordinal> for Pushover {
    fn apply(&self, ballot: Ordinal) -> Ordinal {
        let mut pushover_ranking: Vec<Id> = (*ballot).clone();
        pushover_ranking.retain(|c| !self.0.contains(c) && !self.1.contains(c));
        let mut ranking = self.0.clone();
        ranking.append(&mut self.1.clone());
        ranking.append(&mut pushover_ranking);
        Ordinal(ranking)
    }
}
