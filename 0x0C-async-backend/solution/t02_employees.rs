//! Task 2: the employee endpoints.
//!
//! `GET /employees/{staff_number}` → the employee as JSON, or `NotFound("employee <number>")`.
//!
//! `POST /employees` with a `NewEmployee` body → **201 Created** and the stored `Employee`. Check
//! first, in this order:
//! - `staff_number` is above 0 → else `Invalid("staff number must be above 0")`
//! - `name` and `department`, trimmed, aren't empty → else `Invalid("name can't be blank")` /
//!   `Invalid("department can't be blank")`
//! - `station` is one of `STATIONS` → else `Invalid("unknown station <code>")`
//! - the staff number isn't taken → else `Conflict("staff number <number> already exists")`
//!
//! Store the name and department trimmed. A poisoned lock → `Internal(...)`. Never hold the lock
//! across an `.await` (there's no need to await anything here at all).

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::model::{AppState, Employee, NewEmployee, STATIONS};
use crate::t01_api_error::ApiError;

fn poisoned() -> ApiError {
    ApiError::Internal("the employee store's lock is poisoned".to_string())
}

pub async fn get_employee(
    State(state): State<AppState>,
    Path(staff_number): Path<u32>,
) -> Result<Json<Employee>, ApiError> {
    let employees = state.employees.read().map_err(|_| poisoned())?;
    employees
        .get(&staff_number)
        .cloned()
        .map(Json)
        .ok_or_else(|| ApiError::NotFound(format!("employee {staff_number}")))
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(input): Json<NewEmployee>,
) -> Result<(StatusCode, Json<Employee>), ApiError> {
    if input.staff_number == 0 {
        return Err(ApiError::Invalid(
            "staff number must be above 0".to_string(),
        ));
    }
    let name = input.name.trim();
    if name.is_empty() {
        return Err(ApiError::Invalid("name can't be blank".to_string()));
    }
    let department = input.department.trim();
    if department.is_empty() {
        return Err(ApiError::Invalid("department can't be blank".to_string()));
    }
    if !STATIONS.contains(&input.station.as_str()) {
        return Err(ApiError::Invalid(format!(
            "unknown station {}",
            input.station
        )));
    }

    let employee = Employee {
        staff_number: input.staff_number,
        name: name.to_string(),
        station: input.station,
        department: department.to_string(),
    };

    let mut employees = state.employees.write().map_err(|_| poisoned())?;
    if employees.contains_key(&employee.staff_number) {
        return Err(ApiError::Conflict(format!(
            "staff number {} already exists",
            employee.staff_number
        )));
    }
    employees.insert(employee.staff_number, employee.clone());
    Ok((StatusCode::CREATED, Json(employee)))
}
