//! Task 1: accept input from the screens, and check it.
//!
//! The screens send JSON with camelCase keys: `{"staffNumber": 48213, "code": "DG",
//! "completedOn": "2026-03-01"}`. Make `NewHolding` read that (see the `serde` attributes), then
//! `parse_new_holding` checks it:
//! - JSON that doesn't fit (a missing, unknown or wrongly typed field) →
//!   `Invalid { field: "body", reason: <serde's message> }`
//! - `code`, trimmed, must be 2 to 12 characters of `A`–`Z`, `0`–`9` or `-` →
//!   `Invalid { field: "code", reason: "must be 2 to 12 capital letters, digits or dashes" }`
//! - `completedOn` after `today` → `Invalid { field: "completedOn", reason: "can't be in the future" }`
//!
//! Return the input with `code` trimmed.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;
use serde::Deserialize;

use crate::t00_command_error::CommandError;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
// TODO: make serde read camelCase keys, and reject keys it doesn't know.
pub struct NewHolding {
    pub staff_number: u32,
    pub code: String,
    pub completed_on: NaiveDate,
}

pub fn parse_new_holding(json: &str, today: NaiveDate) -> Result<NewHolding, CommandError> {
    todo!()
}
