//! Task 100 (advanced): `GET /employees` with filters and paging.
//!
//! Query parameters, all optional: `station` (only that station), `after` (only staff numbers
//! above this one), `limit` (how many, default 50, at most 200). Employees come back sorted by
//! staff number. `limit=0` → `Invalid("limit must be at least 1")`; a `limit` above 200 is
//! treated as 200.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

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
    todo!()
}
