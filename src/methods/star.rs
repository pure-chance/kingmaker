use std::collections::HashMap;

use crate::core::*;

/// A single-winner, cardinal voting method. The two candidates with the highest scores advance to a runoff, where the candidate with the most votes in the runoff wins.
#[derive(Debug)]
pub struct Star;

impl Method for Star {
    type Ballot = Cardinal;
    type Winner = SingleWinner;
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        // score candidates
        let mut tally: HashMap<Id, usize> = HashMap::with_capacity(profile.len());
        for ballot in profile.iter() {
            for (id, score) in ballot.0.iter() {
                *tally.entry(*id).or_insert(0) += *score;
            }
        }
        // find top 2 candidates
        let mut sorted_ids: Vec<_> = tally.iter().collect();
        sorted_ids.sort_by_key(|&(_, score)| score);
        sorted_ids.reverse();
        if sorted_ids.len() < 2 {
            return SingleWinner::None;
        }
        let (c1, _) = sorted_ids[0];
        let (c2, _) = sorted_ids[1];
        // instant runoff
        let (c1_tally, c2_tally) =
            profile
                .iter()
                .fold((0, 0), |(c1_tally, c2_tally), preference| {
                    let score1 = preference.0.get(c1).unwrap();
                    let score2 = preference.0.get(c2).unwrap();
                    (
                        c1_tally + (score1 > score2) as usize,
                        c2_tally + (score2 > score1) as usize,
                    )
                });
        let winners = match (c1_tally, c2_tally) {
            (c1_tally, c2_tally) if c1_tally > c2_tally => vec![*c1],
            (c1_tally, c2_tally) if c2_tally > c1_tally => vec![*c2],
            (c1_tally, c2_tally) if c1_tally == c2_tally => vec![*c1, *c2],
            _ => vec![],
        };
        match winners.len() {
            0 => SingleWinner::none(),
            1 => SingleWinner::win(candidates, winners[0]),
            _ => SingleWinner::tie(candidates, &winners),
        }
    }
}
