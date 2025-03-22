use std::collections::HashMap;

use crate::core::*;

///...TODO!
#[derive(Debug)]
pub struct IRV;

impl Method for IRV {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(
        &self,
        candidate_pool: &[Candidate],
        profile: &Profile<Self::Ballot>,
    ) -> Self::Winner {
        let majority = (profile.len() + 1) / 2;
        let mut profile = Profile(profile.iter().cloned().collect());

        let first_place_counts = |profile: &Profile<Self::Ballot>| -> HashMap<Id, usize> {
            profile.iter().fold(HashMap::new(), |mut map, b| {
                let first_place_candidate = (*b).first();
                if first_place_candidate.is_some() {
                    *map.entry((*b)[0]).or_insert(0) += 1;
                }
                map
            })
        };

        let mut fpc = first_place_counts(&profile);
        let mut max_first_place_votes: usize = *fpc.values().max().unwrap();

        while max_first_place_votes < majority {
            let min_first_place_votes: usize = *fpc.values().min().unwrap();
            let losers: Vec<Id> = fpc
                .iter()
                .filter(|&(_, lpv)| *lpv == min_first_place_votes)
                .map(|(id, _)| *id)
                .collect();

            profile = profile
                .iter_mut()
                .filter(|b| !b.is_empty())
                .map(|b| {
                    b.retain(|c| !losers.contains(c));
                    b.clone()
                })
                .collect::<Vec<_>>()
                .into();

            fpc = first_place_counts(&profile);
            max_first_place_votes = *fpc.values().max().unwrap();
        }

        let winners: Vec<Id> = fpc
            .iter()
            .filter(|&(_, fpv)| *fpv == max_first_place_votes)
            .map(|(id, _)| *id)
            .collect();
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidate_pool, winners[0]),
            _ => SingleWinner::tie(candidate_pool, &winners),
        }
    }
}
