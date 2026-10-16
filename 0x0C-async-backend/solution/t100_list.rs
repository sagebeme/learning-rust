//! Task 100 (advanced): `GET /employees` with filters and paging.
//!
//! Query parameters, all optional: `station` (only that station), `after` (only staff numbers
//! above this one), `limit` (how many, default 50, at most 200). Employees come back sorted by
//! staff number. `limit=0` → `Invalid("limit must be at least 1")`; a `limit` above 200 is
//! treated as 200.

use axum::Json;
use axum::extract::{Query, State};
use serde::Deserialize;

use crate::model::{AppState, Employee};
use crate::t01_api_error::ApiError;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListParams {
    pub station: Option<String>,
    pub after: Option<u32>,
    pub limit: Option<usize>,
}

pub async fn list_employees(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Employee>>, ApiError> {
    let limit = params.limit.unwrap_or(50).min(200);
    if limit == 0 {
        return Err(ApiError::Invalid("limit must be at least 1".to_string()));
    }

    let employees = state
        .employees
        .read()
        .map_err(|_| ApiError::Internal("the employee store's lock is poisoned".to_string()))?;
    // A BTreeMap is already sorted by key, so `range` starts exactly after the last page.
    let start = params.after.map_or(0, |after| after.saturating_add(1));
    let page = employees
        .range(start..)
        .map(|(_, employee)| employee)
        .filter(|e| params.station.as_ref().is_none_or(|s| &e.station == s))
        .take(limit)
        .cloned()
        .collect();
    Ok(Json(page))
}
