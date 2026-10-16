//! Task 4: connect the database to the engine.
//!
//! - `record_completion`: someone completed a qualification. Work out `valid_until` with the
//!   engine (`twa_engine::t00_valid_until::valid_until` and the qualification's type from the
//!   catalogue), then store it with `record_holding` from task 1. A code that isn't in the
//!   catalogue → `DbError::UnknownQualification(code)`, and nothing is stored.
//! - `holdings_for`: load one person's holdings as the engine's `Holding`s, sorted by code, ready
//!   for a gap check. The table has no "last used" column yet, so `last_used_on` is `None`.
//!
//! The database never calculates validity itself: the engine is the only place that rule lives.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;
use sqlx::PgPool;
use twa_engine::model::{Catalogue, Holding};
use twa_engine::t00_valid_until::valid_until;

use crate::model::DbError;
use crate::t01_record_holding::record_holding;

pub async fn record_completion(
    pool: &PgPool,
    catalogue: &Catalogue,
    staff_number: i32,
    code: &str,
    completed_on: NaiveDate,
) -> Result<bool, DbError> {
    todo!()
}

pub async fn holdings_for(pool: &PgPool, staff_number: i32) -> Result<Vec<Holding>, DbError> {
    todo!()
}
