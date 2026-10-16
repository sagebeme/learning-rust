//! Task 3: a requirement, including its prerequisites.
//!
//! Start from `best_holding`. If that state is usable, check every prerequisite of the
//! *required* type, the same way (recursively: a prerequisite can be satisfied by an equivalent,
//! and can have prerequisites of its own). If any prerequisite isn't usable, the result is
//! `PrerequisiteMissing { missing }`, with the unusable prerequisite codes sorted, and the same
//! `satisfied_by` as before.
//!
//! An unusable state stays as it is: there's no point checking prerequisites for an expired
//! qualification. The catalogue is assumed to have no prerequisite cycles (task 100 checks that).

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;

use crate::model::{Catalogue, EngineError, Holding, RequirementState};
use crate::t02_best_holding::best_holding;

pub fn requirement_state(
    catalogue: &Catalogue,
    code: &str,
    holdings: &[Holding],
    today: NaiveDate,
) -> Result<(RequirementState, Option<String>), EngineError> {
    todo!()
}
