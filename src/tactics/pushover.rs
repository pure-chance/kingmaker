use crate::core::{Id, Ordinal, Tactic};

/// The pushover tactic places pushover candidates highly, not to get them elected, but to hopefully knock out stronger candidates in early rounds of voting before losing to their true preferences. The `Vec<Id>` specifies the pushover candidates (in order), with the new ranking being `preferred + pushover + others`.
#[derive(Debug)]
pub struct Pushover {
    /// The preferred candidates.
    pub preferred: Vec<Id>,
    /// The pushover candidates.
    pub pushover: Vec<Id>,
}

impl Pushover {
    /// Instantiates a new Pushover tactic.
    #[must_use]
    pub const fn new(preferred: Vec<Id>, pushover: Vec<Id>) -> Self {
        Self {
            preferred,
            pushover,
        }
    }
}

impl Tactic<Ordinal> for Pushover {
    #[inline]
    fn apply(&self, ballot: Ordinal) -> Ordinal {
        let mut pushover_ranking: Vec<Id> = (*ballot).clone();
        pushover_ranking.retain(|c| !self.preferred.contains(c) && !self.pushover.contains(c));
        let mut ranking = self.preferred.clone();
        ranking.append(&mut self.pushover.clone());
        ranking.append(&mut pushover_ranking);
        Ordinal(ranking)
    }
}
