use ordered_float::NotNan;
use serde::Serialize;

use crate::core::Id;

/// A candidate in an election.
///
/// Each candidate has an id, name, (optional) party, and (optional) list of positions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Candidate {
    /// Unique identifier for the candidate
    ///
    /// Exists to make inputting manual ballots easier (Id instead of a name)
    id: Id,
    /// The name of the candidate
    name: &'static str,
    /// The party the candidate is associated with
    party: Option<&'static str>,
    /// The positions the candidate holds
    positions: Option<Vec<NotNan<f32>>>,
}

impl Candidate {
    /// Create a new candidate
    ///
    /// # Panics
    ///
    /// If any position entry is NaN, then a valid position cannot be instantiated, and the function will panic.
    #[must_use]
    pub fn new(
        id: Id,
        name: &'static str,
        party: Option<&'static str>,
        positions: Option<Vec<f32>>,
    ) -> Self {
        let positions = positions.map(|positions| {
            positions
                .into_iter()
                .map(|position| NotNan::new(position).expect("Position entry is NaN"))
                .collect()
        });
        Self {
            id,
            name,
            party,
            positions,
        }
    }
    /// The id of the candidate
    #[must_use]
    pub const fn id(&self) -> Id {
        self.id
    }
    /// The name of the candidate
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }
    /// The party the candidate is associated with
    #[must_use]
    pub const fn party(&self) -> Option<&str> {
        self.party
    }
    /// The positions that the candidate holds
    #[must_use]
    pub fn positions(&self) -> Option<Vec<NotNan<f32>>> {
        self.positions.clone()
    }
}
