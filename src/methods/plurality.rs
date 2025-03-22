use std::collections::HashMap;

use crate::core::*;

/// Plurality, or First-Past-The-Post is a method of voting where the candidate with the most votes (a plurality) wins. If the two or more candidates have the maximum # of votes, then there is a tie.
#[derive(Debug)]
pub struct Plurality;

impl Method for Plurality {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(
        &self,
        candidate_pool: &[Candidate],
        profile: &Profile<Self::Ballot>,
    ) -> Self::Winner {
        let mut tally: HashMap<Id, usize> = HashMap::with_capacity(profile.len());
        profile
            .iter()
            .map(|ballot| ballot.first())
            .filter(|&ballot| ballot.is_some())
            .for_each(|ballot| {
                let candidate = ballot.unwrap();
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
            1 => SingleWinner::win(candidate_pool, winners[0]),
            _ => SingleWinner::tie(candidate_pool, &winners),
        }
    }
}
