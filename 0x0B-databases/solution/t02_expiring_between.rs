//! Task 2: what's expiring between two dates?
//!
//! Every holding with `valid_until` between `from` and `to` (both included), with the holder's
//! name and station. Optionally only one station. Order by `valid_until`, then staff number, then
//! code, so the report is the same every time.

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::model::{DbError, ExpiringRow};

pub async fn expiring_between(
    pool: &PgPool,
    from: NaiveDate,
    to: NaiveDate,
    station_code: Option<&str>,
) -> Result<Vec<ExpiringRow>, DbError> {
    let rows = sqlx::query_as::<_, ExpiringRow>(
        "SELECT e.staff_number, e.name, e.station_code, h.code, h.valid_until
         FROM holdings h
         JOIN employees e ON e.staff_number = h.staff_number
         WHERE h.valid_until BETWEEN $1 AND $2
           AND ($3::text IS NULL OR e.station_code = $3)
         ORDER BY h.valid_until, e.staff_number, h.code",
    )
    .bind(from)
    .bind(to)
    .bind(station_code)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
