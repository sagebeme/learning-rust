//! Task 2: the employee endpoints.
//!
//! `GET /employees/{staff_number}` → the employee as JSON, or `NotFound("employee <number>")`.
//!
//! `POST /employees` with a `NewEmployee` body → **201 Created** and the stored `Employee` (with
//! no codes yet). Check first, in this order:
//! - `staff_number` is above 0 → else `Invalid("staff number must be above 0")`
//! - `name` and `department`, trimmed, aren't empty → else `Invalid("name can't be blank")` /
//!   `Invalid("department can't be blank")`
//! - `station` is one of `STATIONS` → else `Invalid("unknown station <code>")`
//! - the staff number isn't taken → else `Conflict("staff number <number> already exists")`
//!
//! Store the name and department trimmed. A poisoned lock → `Internal(...)`. Never hold the lock
//! across an `.await` (there's no need to await anything here at all).

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::model::{AppState, Employee, NewEmployee, STATIONS};
use crate::t01_api_error::ApiError;

pub async fn get_employee(
    State(state): State<AppState>,
    Path(staff_number): Path<u32>,
) -> Result<Json<Employee>, ApiError> {
    todo!()
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(input): Json<NewEmployee>,
) -> Result<(StatusCode, Json<Employee>), ApiError> {
    todo!()
}
