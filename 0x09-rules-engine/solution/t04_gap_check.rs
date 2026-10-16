//! Task 4: the gap check, the question TWA asks when someone transfers.
//!
//! Look up what the department and role require (`UnknownRole` if there's nothing defined), and
//! return one `GapLine` per required code, **sorted by code, each code once**, with its state and
//! `satisfied_by` from `requirement_state`.
//!
//! The same inputs must give exactly the same report, whatever order the holdings arrive in.

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
    let required = matrix
        .required(department, role)
        .ok_or_else(|| EngineError::UnknownRole {
            department: department.to_string(),
            role: role.to_string(),
        })?;

    let codes: BTreeSet<&str> = required.iter().map(String::as_str).collect();
    let mut lines = Vec::with_capacity(codes.len());
    for code in codes {
        let (state, satisfied_by) = requirement_state(catalogue, code, holdings, today)?;
        lines.push(GapLine {
            code: code.to_string(),
            state,
            satisfied_by,
        });
    }

    Ok(GapReport {
        department: department.to_string(),
        role: role.to_string(),
        lines,
    })
}
