use std::collections::{BTreeMap, HashSet};

use crate::core::*;

/// A multi-winner, ranked voting method. Candidates with the fewest votes are eliminated in each round, and their votes are transferred to the next preference. This process continues until candidates achieve a required quota or all positions are filled.
#[derive(Debug)]
pub struct STV {
    seats: usize,
}

impl STV {
    pub fn new(seats: usize) -> Self {
        Self { seats }
    }
}

impl Method for STV {
    type Ballot = Ordinal;
    type Winner = MultiWinner;

    fn outcome(
        &self,
        candidate_pool: &[Candidate],
        profile: Profile<Self::Ballot>,
    ) -> Self::Winner {
        let mut ballots: Vec<Self::Ballot> = profile.into_iter().collect();
        let mut winners = HashSet::new();
        let droop_quota = ballots.len() / (self.seats + 1) + 1;
        let mut tally: BTreeMap<Id, usize> = BTreeMap::new();

        while winners.len() < self.seats {
            // count first-choice votes
            tally.clear();
            for ballot in &ballots {
                if let Some(&first_choice) = ballot.0.first() {
                    *tally.entry(first_choice).or_insert(0) += 1;
                }
            }

            let elected = tally
                .iter()
                .filter(|&(_, &votes)| votes >= droop_quota)
                .max_by_key(|&(_, &votes)| votes);
            if let Some((winner, _)) = elected {
                // add winner to winners set and transfer votes
                winners.insert(*winner);
                ballots.iter_mut().for_each(|b| b.0.retain(|c| c != winner));
            } else {
                // remove candidate with fewest votes and transfer votes
                let (loser, _) = tally.iter().min_by_key(|&(_, &votes)| votes).unwrap();
                ballots.iter_mut().for_each(|b| b.0.retain(|c| c != loser));
            }
        }

        // NOTE: If there are 2 possible elected candidates for a given round, the last one is elected that round.

        MultiWinner::Elected(
            winners
                .into_iter()
                .filter_map(|id| candidate_pool.iter().find(|c| c.id() == id).cloned())
                .collect(),
        )
    }
}
