use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::ops::Deref;
use std::slice::Iter;

use derive_more::{Deref, DerefMut, From};
use serde::Serialize;

use crate::core::Id;

/// A ballot type that can be cast in an election. A ballot is an expression of a voter's preferences. There are three ballot types: `Nominal`, `Ordinal`, and `Cardinal`.
pub trait Ballot: Debug + Deref + Send + Sync + Clone + Serialize {}

/// Approval ballot: A set of approved candidates
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Serialize)]
pub struct Nominal(pub BTreeSet<Id>);
impl Ballot for Nominal {}

/// Ranked ballot: An ordered list of candidates
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Serialize)]
pub struct Ordinal(pub Vec<Id>);
impl Ballot for Ordinal {}

/// Score ballot: A map of candidates to scores
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Serialize)]
pub struct Cardinal(pub BTreeMap<Id, usize>);
impl Ballot for Cardinal {}

/// A collection of ballots.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From, Serialize)]
pub struct Profile<B: Ballot>(Box<[B]>);

impl<B: Ballot> Profile<B> {
    /// Create a new Profile from a slice
    pub fn new(items: impl IntoIterator<Item = B>) -> Self {
        Self(Box::from(items.into_iter().collect::<Vec<_>>()))
    }
    /// Return an iterator over the elements
    pub fn iter(&self) -> Iter<'_, B> {
        self.0.iter()
    }
    /// Return the number of elements
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Check if the profile is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<B: Ballot> Deref for Profile<B> {
    type Target = [B];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: Ballot> From<Vec<B>> for Profile<B> {
    fn from(ballots: Vec<B>) -> Self {
        Self(Box::from(ballots))
    }
}

impl<'a, B: Ballot> IntoIterator for &'a Profile<B> {
    type Item = B;
    type IntoIter = std::iter::Cloned<std::slice::Iter<'a, B>>;

    fn into_iter(self) -> Self::IntoIter {
        (*self).iter().cloned()
    }
}

impl<B: Ballot> FromIterator<B> for Profile<B> {
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        let all_ballots: Vec<B> = iter.into_iter().collect();
        Self(all_ballots.into_boxed_slice())
    }
}

impl<B: Ballot> FromIterator<Self> for Profile<B> {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        let all_ballots: Vec<B> = iter
            .into_iter()
            .flat_map(|profile| profile.0.into_vec())
            .collect();

        Self(all_ballots.into_boxed_slice())
    }
}
