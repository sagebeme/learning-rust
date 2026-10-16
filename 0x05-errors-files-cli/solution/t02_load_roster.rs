//! Task 2: load a roster from a file.
//!
//! Read the file with `std::fs::read_to_string`. If that fails, return `RosterError::Read` with
//! the path and the original `io::Error`. Then parse it: a `ParseError` turns into a `RosterError`
//! automatically through `?`, because `RosterError` has `#[from] ParseError`.

use std::path::Path;

use crate::model::{RosterError, StaffRecord};
use crate::t01_parse_roster::parse_roster;

pub fn load_roster(path: &Path) -> Result<Vec<StaffRecord>, RosterError> {
    let text = std::fs::read_to_string(path).map_err(|source| RosterError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(parse_roster(&text)?)
}
