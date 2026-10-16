//! Task 100 (advanced): import a roster, all or nothing.
//!
//! Insert every employee inside one transaction, using `insert_employee` from task 0. If any row
//! fails, nothing is saved, and the error is `DbError::InRow { row, source }`, with `row` counting
//! from 1. On success, return how many were imported.

use sqlx::PgPool;

use crate::model::{DbError, NewEmployee};
use crate::t00_insert_employee::insert_employee;

pub async fn import_employees(pool: &PgPool, employees: &[NewEmployee]) -> Result<usize, DbError> {
    let mut tx = pool.begin().await?;
    for (index, employee) in employees.iter().enumerate() {
        insert_employee(&mut *tx, employee)
            .await
            .map_err(|source| DbError::InRow {
                row: index + 1,
                source: Box::new(source),
            })?;
    }
    tx.commit().await?;
    Ok(employees.len())
}
