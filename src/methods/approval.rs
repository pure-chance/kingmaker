use std::collections::HashMap;

use crate::core::*;

/// A single-winner, nominal voting method. The winner is the candidate(s) with the most approvals.
#[derive(Debug)]
pub struct Approval;

impl Method for Approval {
    type Ballot = Nominal;
    type Winner = SingleWinner;
    fn outcome(
        &self,
        candidate_pool: &[Candidate],
        profile: Profile<Self::Ballot>,
    ) -> Self::Winner {
        let mut approval_counts: HashMap<Id, usize> = HashMap::with_capacity(profile.len());
        (0..profile.len()).for_each(|i| {
            for candidate in profile[i].iter() {
                *approval_counts.entry(*candidate).or_insert(0) += 1;
            }
        });
        let max_count = approval_counts.values().max().unwrap();
        let winners: Vec<Id> = approval_counts
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
