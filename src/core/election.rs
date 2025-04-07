use polars::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;
use serde_json::json;

use super::{Ballot, Candidate, Method, Outcome, Profile, VotingBlock};

/// An election is a simulation of the voting process. It is constructed with a set of conditions, a set of candidates, a pool of voters, and a method for determining the winner.
#[derive(Debug)]
pub struct Election<B, C, M>
where
    B: Ballot,
    C: Send + Sync,
    M: Method<Ballot = B>,
{
    conditions: C,
    candidates: Vec<Candidate>,
    voter_pool: Vec<VotingBlock<B>>,
    method: M,
}

impl<B, C, M> Election<B, C, M>
where
    B: Ballot,
    C: Send + Sync,
    M: Method<Ballot = B>,
{
    pub fn new(
        conditions: C,
        candidates: impl IntoIterator<Item = Candidate>,
        voter_pool: impl IntoIterator<Item = VotingBlock<B>>,
        method: M,
    ) -> Self {
        Self {
            conditions,
            candidates: candidates.into_iter().collect(),
            voter_pool: voter_pool.into_iter().collect(),
            method,
        }
    }
    /// Get the conditions of the election.
    pub fn conditions(&self) -> &C {
        &self.conditions
    }
    /// Get the candidates up for election.
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }
    /// Get the pool of voters
    pub fn voter_pool(&self) -> &[VotingBlock<B>] {
        &self.voter_pool
    }
    /// Get the method used to determine the winner of the election.
    pub fn method(&self) -> &M {
        &self.method
    }
    /// Realizes the preferences of the voters into an honest Profile.
    pub fn realize(&self, rng: &mut StdRng) -> Profile<B> {
        Profile::from_iter(
            self.voter_pool()
                .iter()
                .map(|voting_block| voting_block.realize(self.candidates(), rng)),
        )
    }
    /// Realizes the preferences of the voters and implements strategic voting.
    ///
    /// This produces a profile of strategic votes, which is what is submitted for tabulation and outcome determination.
    pub fn vote(&self, rng: &mut StdRng) -> Profile<B> {
        Profile::from_iter(
            self.voter_pool()
                .iter()
                .map(|voting_block| voting_block.vote(self.candidates(), rng)),
        )
    }
    /// Run a single election with the given configuration
    pub fn run_once(&self, seed: u64) -> impl Outcome {
        let mut rng = StdRng::seed_from_u64(seed);
        let profile: Profile<B> = self.vote(&mut rng);
        self.method().outcome(self.candidates(), profile)
    }
    /// Run many elections with the given configuration
    pub fn run_many(&self, iterations: usize, seed: u64) -> Vec<impl Outcome> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..iterations)
            .map(|_| rng.random())
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|seed| self.run_once(seed))
            .collect()
    }
    /// Tabulates the outcomes of the elections.
    pub fn tabulate<O: Outcome>(&self, outcomes: impl IntoIterator<Item = O>) -> Vec<(O, usize)> {
        let mut result: Vec<(O, usize)> = Vec::new();
        for outcome in outcomes {
            match result.iter_mut().find(|(o, _)| o == &outcome) {
                Some((_, count)) => *count += 1,
                None => result.push((outcome, 1)),
            }
        }
        result
    }
    /// Displays the tabulated outcomes of the elections.
    pub fn display<O: Outcome + std::fmt::Display>(&self, outcomes: impl IntoIterator<Item = O>) {
        let tabulated = self.tabulate(outcomes);
        for (outcome, count) in tabulated {
            println!("{}: {}", outcome, count);
        }
    }
}

#[cfg(feature = "visualize")]
impl<B, C, M> Election<B, C, M>
where
    B: Ballot,
    C: Send + Sync + std::fmt::Debug,
    M: Method<Ballot = B> + std::fmt::Debug,
{
    pub fn visualize<O: Outcome>(&self, outcomes: Vec<O>) {
        let tabulated = self.tabulate(outcomes);
        let df = DataFrame::new(vec![
            Column::new(
                "outcome".into(),
                tabulated
                    .iter()
                    .map(|(outcome, _)| format!("{}", outcome))
                    .collect::<Vec<_>>(),
            ),
            Column::new(
                "count".into(),
                tabulated
                    .iter()
                    .map(|(_, count)| format!("{}", count))
                    .collect::<Vec<_>>(),
            ),
        ])
        .unwrap();
        println!("{}", df);
    }
    fn configuration(&self) -> serde_json::Value {
        json!({
            "conditions": format!("{:?}", self.conditions()),
            "candidates": self.candidates(),
            "voting_blocks": self.voter_pool().iter().map(|block| {
                json!({
                    "preferences": format!("{:?}", block.preferences()),
                    "strategy": format!("{:?}", block.strategy()),
                    "members": block.members()
                })
            }).collect::<Vec<_>>(),
            "method": format!("{:?}", self.method()) // only need the name
        })
    }
    fn outcomes<O: Outcome>(self, outcomes: Vec<O>) -> serde_json::Value {
        let tabulated = self.tabulate(outcomes);
        tabulated
            .iter()
            .map(|(winners, times)| json!({ "winners": winners.winners(), "times": times }))
            .collect::<Vec<_>>()
            .into()
    }
    pub fn write<O: Outcome>(self, outcomes: Vec<O>) -> serde_json::Value {
        let configuration = self.configuration();
        let outcomes = self.outcomes(outcomes);
        json!({
            "configuration": configuration,
            "outcomes": outcomes
        })
    }
}
