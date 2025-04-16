use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use serde::Serialize;

use super::{Candidate, Id};

/// The outcome of an election (single-winner or multi-winner)
pub trait Outcome: Send + Sync + Serialize + Debug + Display + Eq {
    fn winners(&self) -> Vec<&str>;
}
impl Outcome for SingleWinner {
    /// Get the winners of the single-winner election
    fn winners(&self) -> Vec<&str> {
        match self {
            SingleWinner::Win(candidate) => vec![candidate.name()],
            SingleWinner::Tie(candidates) => candidates.iter().map(|c| c.name()).collect(),
            SingleWinner::None => vec![],
        }
    }
}
impl Outcome for MultiWinner {
    /// Get the winners of the multi-winner election
    fn winners(&self) -> Vec<&str> {
        match self {
            MultiWinner::Elected(candidates) => candidates.iter().map(|c| c.name()).collect(),
            MultiWinner::None => vec![],
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
    pub fn win(candidates: &[Candidate], id: Id) -> Self {
        SingleWinner::Win(candidates.iter().find(|c| c.id() == id).unwrap().to_owned())
    }
    /// Construct a `SingleWinner::Tie()`
    pub fn tie(candidates: &[Candidate], ids: &[Id]) -> Self {
        SingleWinner::Tie(
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
    pub fn none() -> Self {
        SingleWinner::None
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
    pub fn seats(candidates: &[Candidate], ids: &[Id]) -> Self {
        MultiWinner::Elected(
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
    pub fn none() -> Self {
        MultiWinner::None
    }
}

impl Display for SingleWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleWinner::Win(candidate) => write!(f, "Win({})", candidate.name()),
            SingleWinner::Tie(candidates) => {
                write!(
                    f,
                    "Tie({})",
                    candidates
                        .iter()
                        .map(|c| c.name())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            SingleWinner::None => write!(f, "None"),
        }
    }
}

impl Display for MultiWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiWinner::Elected(candidates) => {
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
            MultiWinner::None => write!(f, "MultiWinner(None)"),
        }
    }
}
