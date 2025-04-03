use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::ops::{Deref, Index};
use std::slice::Iter;

use derive_more::{Deref, DerefMut, From};

use super::Id;

/// A ballot type that can be cast in an election. A ballot is an expression of a voter's preferences. There are three ballot types: `Nominal`, `Ordinal`, and `Cardinal`.
pub trait Ballot: Debug + Deref + Send + Sync + Clone {}

/// Approval ballot: A set of approved candidates
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Nominal(pub BTreeSet<Id>);
impl Ballot for Nominal {}

/// Ranked ballot: An ordered list of candidates
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Ordinal(pub Vec<Id>);
impl Ballot for Ordinal {}

/// Score ballot: A map of candidates to scores
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct Cardinal(pub BTreeMap<Id, usize>);
impl Ballot for Cardinal {}

/// A collection of ballots.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct Profile<B: Ballot>(Box<[B]>);

impl<B: Ballot> Profile<B> {
    /// Create a new Profile from a slice
    pub fn new(items: impl IntoIterator<Item = B>) -> Self {
        Profile(Box::from(items.into_iter().collect::<Vec<_>>()))
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

impl<B: Ballot> Index<usize> for Profile<B> {
    type Output = B;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<B: Ballot> From<Vec<B>> for Profile<B> {
    fn from(ballots: Vec<B>) -> Profile<B> {
        Profile(Box::from(ballots))
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
        Profile(all_ballots.into_boxed_slice())
    }
}

impl<B: Ballot> FromIterator<Profile<B>> for Profile<B> {
    fn from_iter<T: IntoIterator<Item = Profile<B>>>(iter: T) -> Self {
        let all_ballots: Vec<B> = iter
            .into_iter()
            .flat_map(|profile| profile.0.into_vec())
            .collect();

        Profile(all_ballots.into_boxed_slice())
    }
}
