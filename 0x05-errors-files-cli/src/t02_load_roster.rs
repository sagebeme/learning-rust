//! Task 2: load a roster from a file.
//!
//! Read the file with `std::fs::read_to_string`. If that fails, return `RosterError::Read` with
//! the path and the original `io::Error`. Then parse it: a `ParseError` turns into a `RosterError`
//! automatically through `?`, because `RosterError` has `#[from] ParseError`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::path::Path;

use crate::model::{RosterError, StaffRecord};
use crate::t01_parse_roster::parse_roster;

pub fn load_roster(path: &Path) -> Result<Vec<StaffRecord>, RosterError> {
    todo!()
}
