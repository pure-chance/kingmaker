use crate::core::{Candidate, Method, Ordinal, Profile, SingleWinner};
use crate::methods::find_candidates_with_value;

/// A single-winner, ranked voting method. The candidate with the most votes (a plurality) wins.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Plurality;

impl Method for Plurality {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        let first_place_votes =
            profile
                .iter()
                .fold(vec![0usize; candidates.len()], |mut counts, b| {
                    if let Some(first_place_candidate) = b.first() {
                        counts[*first_place_candidate] += 1;
                    }
                    counts
                });
        let max_count = first_place_votes.iter().max().unwrap();
        let winners: Vec<usize> = find_candidates_with_value(&first_place_votes, *max_count);
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
