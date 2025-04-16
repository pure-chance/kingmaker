use std::collections::HashMap;

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

        let first_place_counts = |profile: &Vec<Self::Ballot>| -> HashMap<Id, usize> {
            profile.iter().fold(HashMap::new(), |mut map, b| {
                let first_place_candidate = (*b).first();
                if first_place_candidate.is_some() {
                    *map.entry((*b)[0]).or_insert(0) += 1;
                }
                map
            })
        };

        let mut fpc = first_place_counts(&remaining_ranking);
        let mut max_first_place_votes: usize = *fpc.values().max().unwrap();

        while max_first_place_votes < majority {
            let min_first_place_votes: usize = *fpc.values().min().unwrap();
            let losers: Vec<Id> = fpc
                .iter()
                .filter(|&(_, lpv)| *lpv == min_first_place_votes)
                .map(|(id, _)| *id)
                .collect();

            // if all candidates have the same number of first-place votes, then break and tie
            if losers.len() == fpc.len() {
                break;
            }

            remaining_ranking = remaining_ranking
                .iter_mut()
                .filter(|b| !b.is_empty())
                .map(|b| {
                    b.retain(|c| !losers.contains(c));
                    b.clone()
                })
                .collect::<Vec<_>>();

            fpc = first_place_counts(&remaining_ranking);
            max_first_place_votes = *fpc.values().max().unwrap();
        }

        let winners: Vec<Id> = fpc
            .iter()
            .filter(|&(_, fpv)| *fpv == max_first_place_votes)
            .map(|(id, _)| *id)
            .collect();
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
