//! Task 2: the best holding for a requirement.
//!
//! A requirement for `code` can be satisfied by a holding of `code` itself, or of any code in its
//! `equivalents`. Work out each candidate holding's state **using the holding's own qualification
//! type**, and pick the best one:
//! 1. the lowest `rank()`
//! 2. then the latest valid-until date
//! 3. then the alphabetically first code
//!
//! Return the state and the code of the holding used, or `(Missing, None)` if there's none.
//! Errors: `UnknownQualification` if `code`, or the code of a candidate holding, isn't in the
//! catalogue.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::cmp::Reverse;

use chrono::NaiveDate;

use crate::model::{Catalogue, EngineError, Holding, RequirementState};
use crate::t00_valid_until::valid_until;
use crate::t01_holding_state::holding_state;

pub fn best_holding(
    catalogue: &Catalogue,
    code: &str,
    holdings: &[Holding],
    today: NaiveDate,
) -> Result<(RequirementState, Option<String>), EngineError> {
    todo!()
}
