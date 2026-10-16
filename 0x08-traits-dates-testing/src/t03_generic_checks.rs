//! Task 3: code that works with *any* validity rule.
//!
//! - `is_valid_on` is **generic**: it works for any type that implements `ValidityRule`. Valid
//!   means `today` is on or before `valid_until`, and not before `completed_on`.
//! - `strictest` takes a list of **trait objects** (rules of different types in one list) and
//!   returns the earliest `valid_until` among them, or `None` for an empty list. When a
//!   qualification is covered by several rules (company policy and the regulator), the strictest
//!   one wins.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;

use crate::t02_validity_rules::ValidityRule;

pub fn is_valid_on<R: ValidityRule + ?Sized>(
    rule: &R,
    completed_on: NaiveDate,
    today: NaiveDate,
) -> bool {
    todo!()
}

pub fn strictest(rules: &[Box<dyn ValidityRule>], completed_on: NaiveDate) -> Option<NaiveDate> {
    todo!()
}
