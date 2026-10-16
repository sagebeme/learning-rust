//! Task 0: add an employee, and explain the failures.
//!
//! Insert into `employees` with `$1`..`$4` placeholders and `.bind(...)`. If Postgres refuses:
//! - a unique violation (the staff number is taken) → `DbError::DuplicateStaffNumber`
//! - a foreign key violation (the station doesn't exist) → `DbError::UnknownStation`
//! - anything else → `DbError::Sqlx`
//!
//! `insert_employee` takes any `PgExecutor`: a pool, a connection, or a transaction (task 100
//! uses it inside one).

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use sqlx::PgExecutor;

use crate::model::{DbError, NewEmployee};

pub async fn insert_employee<'e>(
    executor: impl PgExecutor<'e>,
    employee: &NewEmployee,
) -> Result<(), DbError> {
    todo!()
}
