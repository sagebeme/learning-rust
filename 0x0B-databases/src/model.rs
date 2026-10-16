//! Given: the types and the error every task uses.

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewEmployee {
    pub staff_number: i32,
    pub name: String,
    pub station_code: String,
    pub department: String,
}

impl NewEmployee {
    pub fn new(staff_number: i32, name: &str, station_code: &str, department: &str) -> NewEmployee {
        NewEmployee {
            staff_number,
            name: name.to_string(),
            station_code: station_code.to_string(),
            department: department.to_string(),
        }
    }
}

/// One row of the "what's expiring" report. The field names match the selected columns.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ExpiringRow {
    pub staff_number: i32,
    pub name: String,
    pub station_code: String,
    pub code: String,
    pub valid_until: NaiveDate,
}

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("staff number {0} already exists")]
    DuplicateStaffNumber(i32),
    #[error("station {0:?} does not exist")]
    UnknownStation(String),
    #[error("staff member {0} was not found")]
    NotFound(i32),
    #[error("row {row}: {source}")]
    InRow {
        row: usize,
        #[source]
        source: Box<DbError>,
    },
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
