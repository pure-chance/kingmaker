use ordered_float::OrderedFloat;

use crate::core::{Candidate, Id, Method, MultiWinner, Ordinal, Profile};

/// A multi-winner, ranked voting method. Candidates with the fewest votes are eliminated in each round, and their votes are transferred to the next preference. This process continues until candidates achieve a required quota or all positions are filled.
#[derive(Debug, Clone, serde::Serialize)]
pub struct STV {
    seats: usize,
}

impl STV {
    /// Creates a new instance of STV with the specified number of seats.
    #[must_use]
    pub const fn new(seats: usize) -> Self {
        Self { seats }
    }
}

impl Method for STV {
    type Ballot = Ordinal;
    type Winner = MultiWinner;
    #[inline]
    fn outcome(&self, candidates: &[Candidate], profile: Profile<Self::Ballot>) -> Self::Winner {
        let mut ballots: Vec<Self::Ballot> = profile.into_iter().collect();
        let mut winners: Vec<Id> = vec![];
        let droop_quota = OrderedFloat((ballots.len() / (self.seats + 1) + 1) as f64);

        let mut vote_shares = profile.iter().fold(
            vec![OrderedFloat(0.0); candidates.len()],
            |mut counts, b| {
                if let Some(first_place_candidate) = b.first() {
                    counts[*first_place_candidate] += OrderedFloat(1.0);
                }
                counts
            },
        );

        while winners.len() < self.seats {
            // determine elected candidates (if any)
            let elected = vote_shares.iter().position(|&votes| votes >= droop_quota);
            if let Some(winner) = elected {
                // NOTE: It's fine to have one winner elected at a time, as if there are multiple winners, they will be elected one by one. If the seats fill up, then there needs to be a mechanism to select which candidates will be elected, and that mechanism is built in.

                // Add winner to winners set and proportionally transfer surplus votes.
                winners.push(winner);

                let surplus = vote_shares[winner] - droop_quota;
                vote_shares[winner] = droop_quota;

                let mut second_place_proportions = vec![0usize; candidates.len()];
                ballots
                    .iter()
                    .filter(|b| b.first() == Some(&winner))
                    .for_each(|b| {
                        let second_place = b.get(1);
                        if let Some(second_place) = second_place {
                            second_place_proportions[*second_place] += 1;
                        }
                    });
                second_place_proportions
                    .iter()
                    .enumerate()
                    .for_each(|(c, &count)| {
                        vote_shares[c] += OrderedFloat(count as f64) / surplus;
                    });

                ballots.retain(|b| b.first() != Some(&winner));
            } else {
                // Remove candidate with fewest votes and proportionally transfer votes.
                let min_votes = vote_shares.iter().min().unwrap();
                let losers: Vec<usize> = vote_shares
                    .iter()
                    .enumerate()
                    .filter_map(|(id, votes)| (votes == min_votes).then_some(id))
                    .collect();
                ballots
                    .iter_mut()
                    .for_each(|b| b.0.retain(|c| !losers.contains(c)));
            }
        }

        MultiWinner::Elected(
            winners
                .into_iter()
                .filter_map(|id| candidates.iter().find(|c| c.id() == id).cloned())
                .collect(),
        )
    }
}
