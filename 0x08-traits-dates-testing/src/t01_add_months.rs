//! Task 1: add calendar months, the way people mean it.
//!
//! "12 months after 15 March" is 15 March next year. When the day doesn't exist in the target
//! month, use the last day of that month: 31 January + 1 month is 28 February (29 in a leap
//! year), and 29 February 2024 + 12 months is 28 February 2025.
//!
//! Dates can't go past chrono's `NaiveDate::MAX`; if they would, return `NaiveDate::MAX`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::{Months, NaiveDate};

pub fn add_months(date: NaiveDate, months: u32) -> NaiveDate {
    todo!()
}
