use std::collections::HashMap;

use crate::core::*;

/// A single-winner, ranked voting method. Each rank (in the ballot) is given a value corresponding to its order. With n candidates, 1st is `n-1`, 2nd is `n-2`, and so on, with last having a value of `0`. The candidate whose sum of ranks is highest is the winner.
#[derive(Debug)]
pub struct Borda;

impl Method for Borda {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(
        &self,
        candidate_pool: &[Candidate],
        profile: &Profile<Self::Ballot>,
    ) -> Self::Winner {
        let mut tally: HashMap<Id, usize> = HashMap::with_capacity(profile.len());
        (0..profile.len()).for_each(|i| {
            for (i, candidate) in profile[i].iter().enumerate() {
                *tally.entry(*candidate).or_insert(0) += profile[i].len() - i;
            }
        });
        let max_count = tally.values().max().unwrap();
        let winners: Vec<Id> = tally
            .iter()
            .filter(|(_, count)| count == &max_count)
            .map(|(id, _)| *id)
            .collect();
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidate_pool, winners[0]),
            _ => SingleWinner::tie(candidate_pool, &winners),
        }
    }
}
