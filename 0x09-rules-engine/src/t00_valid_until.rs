//! Task 0: the last day a holding is valid.
//!
//! `validity_months` calendar months after `completed_on`, clamped at month ends (31 Jan + 1
//! month is 28 or 29 Feb). If that would pass the largest date chrono can store, use
//! `NaiveDate::MAX`. (You wrote this in 0x08.)

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::{Months, NaiveDate};

use crate::model::QualificationType;

pub fn valid_until(qualification: &QualificationType, completed_on: NaiveDate) -> NaiveDate {
    todo!()
}
