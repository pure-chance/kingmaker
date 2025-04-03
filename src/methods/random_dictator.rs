use crate::core::*;

/// A single-winner ranked voting method. The winner is determined by selecting a random ballot and returning the winner(s) of that ballot. That ballot is the only ballot that matters, hence the title "random dictator".
#[derive(Debug)]
pub struct RandomDictator;

impl Method for RandomDictator {
    type Ballot = Ordinal;
    type Winner = SingleWinner;
    fn outcome(
        &self,
        candidate_pool: &[Candidate],
        profile: Profile<Self::Ballot>,
    ) -> Self::Winner {
        if profile.is_empty() {
            return SingleWinner::None;
        }
        let winner = profile[0][0];
        SingleWinner::win(candidate_pool, winner)
    }
}
