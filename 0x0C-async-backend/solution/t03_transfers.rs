//! Task 3: can this person transfer?
//!
//! `POST /transfers/check` with `{"staff_number": 20020, "department": "Cargo", "role": "Handler"}`
//! → `{"ready": false, "missing": ["DG", "FORKLIFT"]}`: the required codes the employee doesn't
//! hold, sorted, each once.
//! - unknown employee → `NotFound("employee <number>")`
//! - no requirements for that department and role → `Invalid("no requirements for <role> in
//!   <department>")`
//!
//! (The real server calls the 0x09 engine here, with holdings from the 0x0B database.)

use std::collections::BTreeSet;

use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::model::AppState;
use crate::t01_api_error::ApiError;

#[derive(Debug, Clone, Deserialize)]
pub struct TransferCheck {
    pub staff_number: u32,
    pub department: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TransferAnswer {
    pub ready: bool,
    pub missing: Vec<String>,
}

pub async fn check_transfer(
    State(state): State<AppState>,
    Json(check): Json<TransferCheck>,
) -> Result<Json<TransferAnswer>, ApiError> {
    let held: BTreeSet<String> = {
        let employees = state
            .employees
            .read()
            .map_err(|_| ApiError::Internal("the employee store's lock is poisoned".to_string()))?;
        let employee = employees
            .get(&check.staff_number)
            .ok_or_else(|| ApiError::NotFound(format!("employee {}", check.staff_number)))?;
        employee.codes.iter().cloned().collect()
    }; // the lock is released here, before anything else happens

    let required = state
        .requirements
        .get(&(check.department.clone(), check.role.clone()))
        .ok_or_else(|| {
            ApiError::Invalid(format!(
                "no requirements for {} in {}",
                check.role, check.department
            ))
        })?;

    let missing: BTreeSet<&String> = required.iter().filter(|c| !held.contains(*c)).collect();
    Ok(Json(TransferAnswer {
        ready: missing.is_empty(),
        missing: missing.into_iter().cloned().collect(),
    }))
}
