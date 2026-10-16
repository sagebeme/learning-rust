//! Task 0: add an employee, and explain the failures.
//!
//! Insert into `employees` with `$1`..`$4` placeholders and `.bind(...)`. If Postgres refuses:
//! - a unique violation (the staff number is taken) → `DbError::DuplicateStaffNumber`
//! - a foreign key violation (the station doesn't exist) → `DbError::UnknownStation`
//! - anything else → `DbError::Sqlx`
//!
//! `insert_employee` takes any `PgExecutor`: a pool, a connection, or a transaction (task 100
//! uses it inside one).

use sqlx::PgExecutor;

use crate::model::{DbError, NewEmployee};

pub async fn insert_employee<'e>(
    executor: impl PgExecutor<'e>,
    employee: &NewEmployee,
) -> Result<(), DbError> {
    let result = sqlx::query(
        "INSERT INTO employees (staff_number, name, station_code, department)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(employee.staff_number)
    .bind(&employee.name)
    .bind(&employee.station_code)
    .bind(&employee.department)
    .execute(executor)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            Err(DbError::DuplicateStaffNumber(employee.staff_number))
        }
        Err(sqlx::Error::Database(error)) if error.is_foreign_key_violation() => {
            Err(DbError::UnknownStation(employee.station_code.clone()))
        }
        Err(other) => Err(DbError::Sqlx(other)),
    }
}
