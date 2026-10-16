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

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::model::DbError;

pub async fn transfer_employee(
    pool: &PgPool,
    staff_number: i32,
    to_department: &str,
    transferred_on: NaiveDate,
) -> Result<String, DbError> {
    todo!()
}
