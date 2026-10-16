//! Task 3: can this person transfer? The server asks your engine.
//!
//! `POST /transfers/check` with
//! `{"staff_number": 48213, "department": "Cargo", "role": "Handler", "on": "2026-10-19"}`
//! → `{"ready": false, "report": <the engine's GapReport>}`.
//!
//! 1. an unknown employee → `NotFound("employee <number>")`
//! 2. copy their holdings out of the state (none recorded → an empty list), and let go of the lock
//! 3. call `twa_engine::t04_gap_check::gap_check` with the state's catalogue and matrix, for the
//!    date `on` (the day the transfer would take effect: the server never guesses "today")
//! 4. engine errors: `UnknownRole` → `Invalid(<the engine error's message>)`;
//!    `UnknownQualification` → `Internal(<the message>)`

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use axum::Json;
use axum::extract::State;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use twa_engine::model::{EngineError, GapReport};
use twa_engine::t04_gap_check::gap_check;

use crate::model::AppState;
use crate::t01_api_error::ApiError;

#[derive(Debug, Clone, Deserialize)]
pub struct TransferCheck {
    pub staff_number: u32,
    pub department: String,
    pub role: String,
    pub on: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TransferAnswer {
    pub ready: bool,
    pub report: GapReport,
}

pub async fn check_transfer(
    State(state): State<AppState>,
    Json(check): Json<TransferCheck>,
) -> Result<Json<TransferAnswer>, ApiError> {
    todo!()
}
