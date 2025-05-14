use crate::core::{Candidate, Cardinal, Method, Profile, SingleWinner};

/// A single-winner, cardinal voting method. The two candidates with the highest scores advance to a runoff, where the candidate with the most votes in the runoff wins.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Star;

impl Method for Star {
    type Ballot = Cardinal;
    type Winner = SingleWinner;
    #[inline]
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        // Score candidates
        let cumulative_scores = profile.iter().fold(
            vec![0usize; candidates.len()],
            |mut cumulative_scores, b| {
                for (id, score) in b.iter() {
                    cumulative_scores[*id] += score;
                }
                cumulative_scores
            },
        );

        let (mut first_place_score, mut second_place_score) = (0, 0);
        let (mut first_place, mut second_place) = (0, 0);
        for (i, score) in cumulative_scores.iter().enumerate() {
            if *score > first_place_score {
                second_place_score = first_place_score;
                second_place = first_place;
                first_place_score = *score;
                first_place = i;
            } else if *score > second_place_score {
                second_place_score = *score;
                second_place = i;
            }
        }

        // Instant runoff (tally who has the most wins (higher placements))
        let (first_tally, second_tally) = profile.iter().fold((0, 0), |(c1, c2), preference| {
            let first_score = preference.0.get(&first_place).unwrap();
            let second_score = preference.0.get(&second_place).unwrap();
            if first_score > second_score {
                (c1 + 1, c2)
            } else if second_score > first_score {
                (c1, c2 + 1)
            } else {
                (c1, c2)
            }
        });
        match (first_tally, second_tally) {
            (first_tally, second_tally) if first_tally > second_tally => {
                SingleWinner::win(candidates, first_place)
            }
            (first_tally, second_tally) if second_tally > first_tally => {
                SingleWinner::win(candidates, second_place)
            }
            (first_tally, second_tally) if first_tally == second_tally => {
                SingleWinner::tie(candidates, &[first_place, second_place])
            }
            _ => SingleWinner::none(),
        }
    }
}
