//! Given: the engine's data model. Read it closely before starting; you don't need to change it.

use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A kind of qualification, as the training department defines it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationType {
    pub code: String,
    pub name: String,
    /// Valid for this many calendar months after completion (clamped at month ends).
    pub validity_months: u32,
    /// Days after expiry during which the holder may still work.
    pub grace_days: u32,
    /// Start warning this many days before expiry.
    pub warn_days: u32,
    /// If set: the holder must have *used* the qualification within this many days to stay current.
    pub recency_days: Option<u32>,
    /// Codes that must also be usable for this one to count.
    pub prerequisites: Vec<String>,
    /// Codes that satisfy a requirement for this one (for example an external equivalent).
    pub equivalents: Vec<String>,
}

/// One person's record of one qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holding {
    pub code: String,
    pub completed_on: NaiveDate,
    /// When they last used it. `None` means "not recorded": count from `completed_on`.
    pub last_used_on: Option<NaiveDate>,
}

/// Where one requirement stands. Ordered from best to worst in `rank()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum RequirementState {
    Met {
        valid_until: NaiveDate,
    },
    ExpiringSoon {
        valid_until: NaiveDate,
        days_left: i64,
    },
    InGrace {
        valid_until: NaiveDate,
        days_overdue: i64,
    },
    /// Still within validity, but not used recently enough.
    Lapsed {
        last_used_on: NaiveDate,
    },
    Expired {
        valid_until: NaiveDate,
    },
    /// The holding is usable, but one or more prerequisites aren't.
    PrerequisiteMissing {
        missing: Vec<String>,
    },
    /// Not held at all, and no equivalent is held either.
    Missing,
}

impl RequirementState {
    /// Lower is better. Used to choose between several holdings that could satisfy a requirement.
    pub fn rank(&self) -> u8 {
        match self {
            RequirementState::Met { .. } => 0,
            RequirementState::ExpiringSoon { .. } => 1,
            RequirementState::InGrace { .. } => 2,
            RequirementState::Lapsed { .. } => 3,
            RequirementState::Expired { .. } => 4,
            RequirementState::PrerequisiteMissing { .. } => 5,
            RequirementState::Missing => 6,
        }
    }

    /// Can the person do the work under this requirement today?
    pub fn is_usable(&self) -> bool {
        matches!(
            self,
            RequirementState::Met { .. }
                | RequirementState::ExpiringSoon { .. }
                | RequirementState::InGrace { .. }
        )
    }
}

/// Every qualification type the company uses, by code.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Catalogue {
    pub types: BTreeMap<String, QualificationType>,
}

impl Catalogue {
    /// Builds a catalogue without checking it. Task 100 writes the checked version.
    pub fn from_types(types: Vec<QualificationType>) -> Catalogue {
        Catalogue {
            types: types.into_iter().map(|t| (t.code.clone(), t)).collect(),
        }
    }

    pub fn get(&self, code: &str) -> Option<&QualificationType> {
        self.types.get(code)
    }
}

/// Which qualification codes each department and role requires.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RequirementsMatrix {
    /// department → role → required codes
    pub roles: BTreeMap<String, BTreeMap<String, Vec<String>>>,
}

impl RequirementsMatrix {
    pub fn required(&self, department: &str, role: &str) -> Option<&[String]> {
        self.roles.get(department)?.get(role).map(Vec::as_slice)
    }
}

/// One line of a gap report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GapLine {
    pub code: String,
    pub state: RequirementState,
    /// The code of the holding that was used, if any (it may be an equivalent).
    pub satisfied_by: Option<String>,
}

/// The result of checking someone against a role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GapReport {
    pub department: String,
    pub role: String,
    /// One line per required code, sorted by code.
    pub lines: Vec<GapLine>,
}

impl GapReport {
    /// True when the person can start in the role today.
    pub fn is_ready(&self) -> bool {
        self.lines.iter().all(|line| line.state.is_usable())
    }
}

/// Things the engine refuses to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    UnknownRole { department: String, role: String },
    UnknownQualification(String),
}

/// Problems found when checking a catalogue (task 100).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogueError {
    DuplicateCode(String),
    UnknownReference { code: String, refers_to: String },
    PrerequisiteCycle(Vec<String>),
}

// Written by hand, to show what `thiserror` (0x05) generates for you.
impl std::error::Error for EngineError {}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::UnknownRole { department, role } => {
                write!(f, "no requirements defined for {role} in {department}")
            }
            EngineError::UnknownQualification(code) => {
                write!(f, "qualification {code:?} is not in the catalogue")
            }
        }
    }
}
