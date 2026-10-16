//! Task 3: a transfer, all or nothing.
//!
//! In one transaction:
//! 1. read the employee's current department with `SELECT ... FOR UPDATE` (`NotFound` if there's
//!    no such employee)
//! 2. update their department
//! 3. insert a `transfers` row recording from, to and the date
//! 4. commit
//!
//! If any step fails, nothing must change. Return the department they moved from.

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::model::DbError;

pub async fn transfer_employee(
    pool: &PgPool,
    staff_number: i32,
    to_department: &str,
    transferred_on: NaiveDate,
) -> Result<String, DbError> {
    let mut tx = pool.begin().await?;

    // FOR UPDATE locks this employee's row until the transaction ends, so two transfers of the
    // same person can't interleave.
    let from_department: String =
        sqlx::query_scalar("SELECT department FROM employees WHERE staff_number = $1 FOR UPDATE")
            .bind(staff_number)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(DbError::NotFound(staff_number))?;

    sqlx::query("UPDATE employees SET department = $2 WHERE staff_number = $1")
        .bind(staff_number)
        .bind(to_department)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO transfers (staff_number, from_department, to_department, transferred_on)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(staff_number)
    .bind(&from_department)
    .bind(to_department)
    .bind(transferred_on)
    .execute(&mut *tx)
    .await?;

    // Returning early with `?` anywhere above drops `tx` without committing, which rolls back.
    tx.commit().await?;
    Ok(from_department)
}
