//! Task 4: the gap check, the question TWA asks when someone transfers.
//!
//! Look up what the department and role require (`UnknownRole` if there's nothing defined), and
//! return one `GapLine` per required code, **sorted by code, each code once**, with its state and
//! `satisfied_by` from `requirement_state`.
//!
//! The same inputs must give exactly the same report, whatever order the holdings arrive in.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::model::{Catalogue, EngineError, GapLine, GapReport, Holding, RequirementsMatrix};
use crate::t03_requirement_state::requirement_state;

pub fn gap_check(
    catalogue: &Catalogue,
    matrix: &RequirementsMatrix,
    department: &str,
    role: &str,
    holdings: &[Holding],
    today: NaiveDate,
) -> Result<GapReport, EngineError> {
    todo!()
}
