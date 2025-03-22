use std::collections::BTreeSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use serde::Serialize;

use super::{Candidate, Id};

pub trait Outcome: Send + Sync + Serialize + Debug + Display + Eq {
    fn winners(&self) -> Vec<&str>;
}
impl Outcome for SingleWinner {
    fn winners(&self) -> Vec<&str> {
        match self {
            SingleWinner::Win(candidate) => vec![candidate.name()],
            SingleWinner::Tie(candidates) => candidates.iter().map(|c| c.name()).collect(),
            SingleWinner::None => vec![],
        }
    }
}
impl Outcome for MultiWinner {
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
    pub fn win(candidate_pool: &[Candidate], id: Id) -> Self {
        SingleWinner::Win(
            candidate_pool
                .iter()
                .find(|c| c.id() == id)
                .unwrap()
                .to_owned(),
        )
    }
    pub fn tie(candidate_pool: &[Candidate], ids: &[Id]) -> Self {
        SingleWinner::Tie(
            ids.iter()
                .map(|id| {
                    candidate_pool
                        .iter()
                        .find(|c| &c.id() == id)
                        .unwrap()
                        .to_owned()
                })
                .collect(),
        )
    }
    pub fn none() -> Self {
        SingleWinner::None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum MultiWinner {
    Elected(BTreeSet<Candidate>),
    None,
}

impl MultiWinner {
    pub fn seats(candidate_pool: &[Candidate], ids: &[Id]) -> Self {
        MultiWinner::Elected(
            ids.iter()
                .map(|id| {
                    candidate_pool
                        .iter()
                        .find(|c| &c.id() == id)
                        .unwrap()
                        .to_owned()
                })
                .collect(),
        )
    }
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
