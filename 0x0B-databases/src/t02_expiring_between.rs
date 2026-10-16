//! Task 2: what's expiring between two dates?
//!
//! Every holding with `valid_until` between `from` and `to` (both included), with the holder's
//! name and station. Optionally only one station. Order by `valid_until`, then staff number, then
//! code, so the report is the same every time.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::model::{DbError, ExpiringRow};

pub async fn expiring_between(
    pool: &PgPool,
    from: NaiveDate,
    to: NaiveDate,
    station_code: Option<&str>,
) -> Result<Vec<ExpiringRow>, DbError> {
    todo!()
}
