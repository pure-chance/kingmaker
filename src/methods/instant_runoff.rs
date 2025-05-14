use crate::core::{Candidate, Id, Method, Ordinal, Profile, SingleWinner};

/// A single-winner, ranked voting method. The candidate with the fewest votes is eliminated in each round, and votes for the eliminated candidate are redistributed to the next preference. This process continues until one candidate has a majority.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IRV;

impl Method for IRV {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        let majority = profile.len() / 2 + 1;
        let mut remaining_ranking: Vec<Self::Ballot> = profile.iter().cloned().collect();

        let first_place_counts = |profile: &Vec<Self::Ballot>| -> Vec<usize> {
            profile
                .iter()
                .fold(vec![0; candidates.len()], |mut fpv, b| {
                    if let Some(first_place_candidate) = b.first() {
                        fpv[*first_place_candidate] += 1;
                    }
                    fpv
                })
        };

        let mut fpc = first_place_counts(&remaining_ranking);
        let mut max_first_place_votes: usize = *fpc.iter().max().unwrap();

        while max_first_place_votes < majority {
            let min_first_place_votes: usize = *fpc.iter().min().unwrap();
            let losers: Vec<Id> = fpc
                .iter()
                .enumerate()
                .filter_map(|(i, x)| (*x == min_first_place_votes).then(|| i))
                .collect();

            // if all candidates have the same number of first-place votes, then break and tie
            if losers.len() == fpc.len() {
                break;
            }

            // reallocate votes
            remaining_ranking.retain(|b| !b.is_empty());
            for ballot in &mut remaining_ranking {
                ballot.retain(|c| !losers.contains(c));
            }

            // recalculate the standings
            fpc = first_place_counts(&remaining_ranking);
            max_first_place_votes = *fpc.iter().max().unwrap();
        }

        let winners: Vec<Id> = fpc
            .iter()
            .enumerate()
            .filter_map(|(i, x)| (*x == max_first_place_votes).then(|| i))
            .collect();

        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
