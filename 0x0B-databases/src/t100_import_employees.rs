//! Task 100 (advanced): import a roster, all or nothing.
//!
//! Insert every employee inside one transaction, using `insert_employee` from task 0. If any row
//! fails, nothing is saved, and the error is `DbError::InRow { row, source }`, with `row` counting
//! from 1. On success, return how many were imported.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use sqlx::PgPool;

use crate::model::{DbError, NewEmployee};
use crate::t00_insert_employee::insert_employee;

pub async fn import_employees(pool: &PgPool, employees: &[NewEmployee]) -> Result<usize, DbError> {
    todo!()
}
