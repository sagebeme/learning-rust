//! Task 1: record a qualification, never going backwards.
//!
//! Insert the holding, or if this person already has this code, update it, but **only if the
//! new completion date is later** than the stored one. A late sync from an offline station must
//! never replace a renewal with the older certificate.
//!
//! Return `true` if a row was inserted or updated, `false` if the stored record was newer.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::model::DbError;

pub async fn record_holding(
    pool: &PgPool,
    staff_number: i32,
    code: &str,
    completed_on: NaiveDate,
    valid_until: NaiveDate,
) -> Result<bool, DbError> {
    todo!()
}
