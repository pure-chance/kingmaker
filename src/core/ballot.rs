use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::ops::Deref;

use derive_more::{Deref, DerefMut, From, IntoIterator};

use super::Id;

/// A collection of ballots.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, IntoIterator, From)]
pub struct Profile<B: Ballot>(pub Vec<B>);

impl<B: Ballot + Ord> Profile<B> {
    /// Create a new profile from a vector of ballots.
    pub fn new(ballots: Vec<B>) -> Self {
        Profile(ballots)
    }
    pub fn tally(&self) -> BTreeMap<B, usize> {
        let mut counts = BTreeMap::new();
        for ballot in self.iter() {
            *counts.entry(ballot.clone()).or_insert(0) += 1;
        }
        counts
    }
}

/// A ballot type that can be cast in an election. A ballot is an expression of a voter's preferences. There are three ballot types: `Nominal`, `Ordinal`, and `Cardinal`.
pub trait Ballot: Debug + Deref + Send + Sync + Clone {}

/// Approval ballot: A set of approved candidates
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Nominal(pub BTreeSet<Id>);

/// Ranked ballot: An ordered list of candidates
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Ordinal(pub Vec<Id>);

/// Score ballot: A map of candidates to scores
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Cardinal(pub BTreeMap<Id, usize>);

impl Ballot for Nominal {}
impl Ballot for Ordinal {}
impl Ballot for Cardinal {}
