#[cfg(feature = "visualize")]
use polars::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;
use serde_json::json;

use super::{Ballot, Candidate, Method, Outcome, Profile, VotingBloc};

/// An election is a simulation of the voting process. It is constructed with a set of conditions, a set of candidates, a set of voting blocs, and a method for determining the winner.
#[derive(Debug)]
pub struct Election<B, M>
where
    B: Ballot,
    M: Method<Ballot = B>,
{
    candidates: Vec<Candidate>,
    voting_blocs: Vec<VotingBloc<B>>,
    method: M,
}

impl<B, M> Election<B, M>
where
    B: Ballot,
    M: Method<Ballot = B>,
{
    pub fn new(
        candidates: impl IntoIterator<Item = Candidate>,
        voting_blocs: impl IntoIterator<Item = VotingBloc<B>>,
        method: M,
    ) -> Self {
        Self {
            candidates: candidates.into_iter().collect(),
            voting_blocs: voting_blocs.into_iter().collect(),
            method,
        }
    }
    /// Get the candidates up for election.
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }
    /// Get the voting blocs
    pub fn voting_blocs(&self) -> &[VotingBloc<B>] {
        &self.voting_blocs
    }
    /// Get the method used to determine the winner of the election.
    pub fn method(&self) -> &M {
        &self.method
    }
    /// Realizes the preferences of the voters into an honest Profile.
    pub fn realize(&self, rng: &mut StdRng) -> Profile<B> {
        Profile::from_iter(
            self.voting_blocs()
                .iter()
                .map(|voting_bloc| voting_bloc.realize(self.candidates(), rng)),
        )
    }
    /// Realizes the preferences of the voters and implements strategic voting.
    ///
    /// This produces a profile of strategic votes, which is what is submitted for tabulation and outcome determination.
    pub fn vote(&self, rng: &mut StdRng) -> Profile<B> {
        Profile::from_iter(
            self.voting_blocs()
                .iter()
                .map(|voting_bloc| voting_bloc.vote(self.candidates(), rng)),
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
            match result.iter_mut().find(|(o, _)| *o == outcome) {
                Some((_, count)) => *count += 1,
                None => result.push((outcome, 1)),
            }
        }
        result
    }
    pub fn display<O: Outcome>(&self, outcomes: &[O]) {
        let tabulated = self.tabulate(outcomes.iter().cloned());
        println!("{:?}", tabulated);
    }
}

impl<B, M> Election<B, M>
where
    B: Ballot,
    M: Method<Ballot = B> + std::fmt::Debug,
{
    /// Visualizes the tabulated outcomes of the elections in a DataFrame.
    #[cfg(feature = "visualize")]
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
            "candidates": self.candidates(),
            "voting_blocs": self.voting_blocs().iter().map(|bloc| {
                json!({
                    "preferences": format!("{:?}", bloc.preferences()),
                    "strategy": format!("{:?}", bloc.strategy()),
                    "members": bloc.members()
                })
            }).collect::<Vec<_>>(),
            "method": format!("{:?}", self.method()) // only need the name
        })
    }
    pub fn outcomes<O: Outcome>(self, outcomes: impl IntoIterator<Item = O>) -> serde_json::Value {
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
