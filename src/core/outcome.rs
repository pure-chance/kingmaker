use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use serde::Serialize;

use crate::core::{Candidate, Id};

/// The outcome of an election (single-winner or multi-winner)
pub trait Outcome: Send + Sync + Clone + Serialize + Debug + Display + Eq + Hash {
    /// Get the winners of the election
    fn winners(&self) -> Vec<&str>;
}

impl Outcome for SingleWinner {
    /// Get the winners of the single-winner election
    fn winners(&self) -> Vec<&str> {
        match self {
            Self::Win(candidate) => vec![candidate.name()],
            Self::Tie(candidates) => candidates.iter().map(Candidate::name).collect(),
            Self::None => vec![],
        }
    }
}

impl Outcome for MultiWinner {
    /// Get the winners of the multi-winner election
    fn winners(&self) -> Vec<&str> {
        match self {
            Self::Elected(candidates) => candidates.iter().map(Candidate::name).collect(),
            Self::None => vec![],
        }
    }
}

/// The outcome of a single-winner election
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum SingleWinner {
    /// A single winner
    Win(Candidate),
    /// A tie between multiple candidates
    Tie(BTreeSet<Candidate>),
    /// No winner
    None,
}
impl SingleWinner {
    /// Construct a `SingleWinner::Win()`
    ///
    /// # Panics
    ///
    /// Panics if the candidate with the given ID is not found in the list of candidates.
    #[must_use]
    pub fn win(candidates: &[Candidate], id: Id) -> Self {
        Self::Win(candidates.iter().find(|c| c.id() == id).unwrap().to_owned())
    }
    /// Construct a `SingleWinner::Tie()`
    ///
    /// # Panics
    ///
    /// Panics if any of the candidate IDs are not found in the list of candidates.
    #[must_use]
    pub fn tie(candidates: &[Candidate], ids: &[Id]) -> Self {
        Self::Tie(
            ids.iter()
                .map(|id| {
                    candidates
                        .iter()
                        .find(|c| &c.id() == id)
                        .unwrap()
                        .to_owned()
                })
                .collect(),
        )
    }
    /// Construct a `SingleWinner::None()`
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }
}

/// The outcome of a multi-winner election
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum MultiWinner {
    /// The elected candidates
    Elected(BTreeSet<Candidate>),
    /// None of the candidates were elected
    None,
}

impl MultiWinner {
    /// Construct a `MultiWinner::Elected()`
    ///
    /// # Panics
    ///
    /// Panics if any of the ids do not correspond to a candidate.
    #[must_use]
    pub fn seats(candidates: &[Candidate], ids: &[Id]) -> Self {
        Self::Elected(
            ids.iter()
                .map(|id| {
                    candidates
                        .iter()
                        .find(|c| &c.id() == id)
                        .unwrap()
                        .to_owned()
                })
                .collect(),
        )
    }
    /// Construct a `MultiWinner::None()`
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }
}

impl Display for SingleWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win(candidate) => write!(f, "Win({})", candidate.name()),
            Self::Tie(candidates) => {
                write!(
                    f,
                    "Tie({})",
                    candidates
                        .iter()
                        .map(Candidate::name)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::None => write!(f, "None"),
        }
    }
}

impl Display for MultiWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Elected(candidates) => {
                write!(
                    f,
                    "MultiWinner({})",
                    candidates
                        .iter()
                        .map(|c| c.name().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::None => write!(f, "MultiWinner(None)"),
        }
    }
}
