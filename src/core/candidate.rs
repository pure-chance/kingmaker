use ordered_float::NotNan;

use serde::Serialize;

use crate::core::Id;

/// A candidate in an election.
///
/// Each candidate has a name and (optionally) a party that they are associated
/// with.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Candidate {
    /// Unique identifier for the candidate
    ///
    /// Exists to make inputting manual ballots easier (Id instead of a name)
    id: Id,
    /// The name of the candidate
    name: String,
    /// The party the candidate is associated with
    party: Option<String>,
    /// The positions the candidate holds
    positions: Option<Vec<NotNan<f32>>>,
}

impl Candidate {
    /// Create a new candidate
    pub fn new(id: Id, name: &str, party: Option<&str>, positions: Option<Vec<f32>>) -> Self {
        let positions = positions.map(|positions| {
            positions
                .into_iter()
                .map(|position| NotNan::new(position).expect("Position entry is NaN"))
                .collect()
        });
        Self {
            id,
            name: name.to_string(),
            party: party.map(|p| p.to_string()),
            positions,
        }
    }
    /// The id of the candidate
    pub fn id(&self) -> Id {
        self.id
    }
    /// The name of the candidate
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The party the candidate is associated with
    pub fn party(&self) -> Option<&str> {
        self.party.as_deref()
    }
    /// The positions that the candidate holds
    pub fn positions(&self) -> Option<&Vec<NotNan<f32>>> {
        self.positions.as_ref()
    }
}
