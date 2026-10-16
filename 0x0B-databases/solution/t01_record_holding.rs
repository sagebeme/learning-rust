//! Task 1: record a qualification, never going backwards.
//!
//! Insert the holding, or if this person already has this code, update it, but **only if the
//! new completion date is later** than the stored one. A late sync from an offline station must
//! never replace a renewal with the older certificate.
//!
//! Return `true` if a row was inserted or updated, `false` if the stored record was newer.

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
    let result = sqlx::query(
        "INSERT INTO holdings (staff_number, code, completed_on, valid_until)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (staff_number, code) DO UPDATE
             SET completed_on = EXCLUDED.completed_on,
                 valid_until = EXCLUDED.valid_until
             WHERE holdings.completed_on < EXCLUDED.completed_on",
    )
    .bind(staff_number)
    .bind(code)
    .bind(completed_on)
    .bind(valid_until)
    .execute(pool)
    .await;

    match result {
        Ok(done) => Ok(done.rows_affected() == 1),
        Err(sqlx::Error::Database(error)) if error.is_foreign_key_violation() => {
            Err(DbError::NotFound(staff_number))
        }
        Err(other) => Err(DbError::Sqlx(other)),
    }
}
