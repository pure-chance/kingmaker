use rand::prelude::*;
use rayon::prelude::*;
use serde_json::json;

use super::{Ballot, Candidate, Id, Method, Outcome, Preference, Profile, Strategy, VotingBlock};

/// An election configuration. This configures the details of the election to simulate, and runs the election. ...TODO
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
    /// Create a new election builder.
    pub fn builder(conditions: C, method: M) -> ElectionBuilder<B, C, M> {
        ElectionBuilder::new(conditions, method)
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
        self.voter_pool()
            .iter()
            .flat_map(|voting_block| {
                voting_block
                    .preferences()
                    .sample(self.candidates(), voting_block.members(), rng)
            })
            .collect::<Vec<_>>()
            .into()
    }
    /// Realizes the preferences of the voters and implements strategic voting.
    ///
    /// This produces a profile of strategic votes, which is what is submitted for tallying votes and determining the outcome.
    pub fn vote(&self, rng: &mut StdRng) -> Profile<B> {
        self.voter_pool()
            .iter()
            .flat_map(|voting_block| {
                let honest_ballots = voting_block.preferences().sample(
                    self.candidates(),
                    voting_block.members(),
                    rng,
                );

                voting_block.strategy().apply_profile(&honest_ballots, rng)
            })
            .collect::<Vec<_>>()
            .into()
    }
    /// Run a single election with the given configuration
    pub fn run_once(&self, seed: u64) -> impl Outcome {
        let mut rng = StdRng::seed_from_u64(seed);
        let profile: Profile<B> = self.vote(&mut rng);
        self.method().outcome(self.candidates(), &profile)
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
}

impl<B, C, M> Election<B, C, M>
where
    B: Ballot,
    C: Send + Sync,
    M: Method<Ballot = B>,
{
    /// Tabulates the outcomes of the elections.
    pub fn tabulate<O: Outcome>(self, outcomes: Vec<O>) -> Vec<(O, usize)> {
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
    pub fn display<O: Outcome + std::fmt::Display>(self, outcomes: Vec<O>) {
        let tabulated = self.tabulate(outcomes);
        for (outcome, count) in tabulated {
            println!("{}: {}", outcome, count);
        }
    }
    fn write_configuration(&self) -> serde_json::Value {
        "".into()
    }
    fn write_outcomes<O: Outcome>(self, outcomes: Vec<O>) -> serde_json::Value {
        let tabulated = self.tabulate(outcomes);
        tabulated
            .iter()
            .map(|(winners, times)| json!({ "winners": winners.winners(), "times": times }))
            .collect::<Vec<_>>()
            .into()
    }
    pub fn write<O: Outcome>(self, outcomes: Vec<O>) -> serde_json::Value {
        let configuration = self.write_configuration();
        let outcomes = self.write_outcomes(outcomes);
        json!({
            "configuration": configuration,
            "outcomes": outcomes
        })
    }
}

pub struct ElectionBuilder<B, C, M>
where
    B: Ballot,
    C: Send + Sync,
    M: Method<Ballot = B>,
{
    conditions: C,
    candidates: Option<Vec<Candidate>>,
    voter_pool: Option<Vec<VotingBlock<B>>>,
    method: M,
}

impl<B, C, M> ElectionBuilder<B, C, M>
where
    B: Ballot,
    C: Send + Sync,
    M: Method<Ballot = B>,
{
    pub fn new(conditions: C, method: M) -> Self {
        Self {
            conditions,
            candidates: None,
            voter_pool: None,
            method,
        }
    }
    pub fn set_conditions(mut self, conditions: C) -> Self {
        self.conditions = conditions;
        self
    }
    pub fn set_method(mut self, method: M) -> Self {
        self.method = method;
        self
    }
    pub fn add_candidate(
        mut self,
        id: Id,
        name: &str,
        party: Option<&str>,
        positions: Option<Vec<f32>>,
    ) -> Self {
        let candidate = Candidate::new(id, name, party, positions);
        self.candidates.get_or_insert_with(Vec::new).push(candidate);
        self
    }
    pub fn add_voting_block(
        mut self,
        preference: impl Preference<B> + 'static,
        strategy: Strategy<B>,
        members: usize,
    ) -> Self {
        let voting_block = VotingBlock::new(preference, strategy, members);
        self.voter_pool
            .get_or_insert_with(Vec::new)
            .push(voting_block);
        self
    }
    pub fn build(self) -> Election<B, C, M> {
        Election {
            conditions: self.conditions,
            candidates: self.candidates.unwrap_or_default(),
            voter_pool: self.voter_pool.unwrap_or_default(),
            method: self.method,
        }
    }
}
