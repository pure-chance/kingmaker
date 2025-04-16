use std::collections::HashMap;

use crate::core::{Candidate, Id, Method, Ordinal, Profile, SingleWinner};

/// A single-winner, ranked voting method. The candidate with the most votes (a plurality) wins.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Plurality;

impl Method for Plurality {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        let mut tally: HashMap<Id, usize> = HashMap::with_capacity(profile.len());
        profile
            .iter()
            .filter_map(|ballot| ballot.first())
            .for_each(|candidate| {
                *tally.entry(*candidate).or_insert(0) += 1;
            });
        let max_count = tally.values().max().unwrap();
        let winners: Vec<Id> = tally
            .iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(candidate, _)| *candidate)
            .collect();
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
