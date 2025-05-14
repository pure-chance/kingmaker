use crate::core::{Candidate, Id, Method, Nominal, Profile, SingleWinner};
use crate::methods::find_candidates_with_value;

/// A single-winner, nominal voting method. The winner is the candidate(s) with the most approvals.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Approval;

impl Method for Approval {
    type Ballot = Nominal;
    type Winner = SingleWinner;
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        let mut approvals = vec![0; candidates.len()];
        profile.iter().for_each(|ballot| {
            for id in ballot.iter() {
                approvals[*id as usize] += 1;
            }
        });
        let max_count = approvals.iter().max().unwrap();
        let winners: Vec<Id> = find_candidates_with_value(&approvals, *max_count);
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
