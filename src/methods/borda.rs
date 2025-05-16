use crate::core::{Candidate, Id, Method, Ordinal, Profile, SingleWinner};
use crate::methods::find_candidates_with_value;

/// A single-winner, ranked voting method. Each rank (in the ballot) is given a value corresponding to its order. With n candidates, 1st is `n-1`, 2nd is `n-2`, and so on, with last having a value of `0`. The candidate whose sum of ranks is highest is the winner.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Borda;

impl Method for Borda {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    #[inline]
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        let ranking_score =
            profile
                .iter()
                .fold(vec![0usize; candidates.len()], |mut ranking_score, b| {
                    for (i, candidate) in b.iter().enumerate() {
                        ranking_score[*candidate] += b.len() - i;
                    }
                    ranking_score
                });
        let max_ranking = ranking_score.iter().max().unwrap();
        let winners: Vec<Id> = find_candidates_with_value(&ranking_score, max_ranking);
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
