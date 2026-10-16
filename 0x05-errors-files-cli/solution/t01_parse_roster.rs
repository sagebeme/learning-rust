//! Task 1: parse a whole roster.
//!
//! The first line is a header (`staff_number,name,station`): skip it. Skip blank lines too.
//! Line numbers count from 1 and include the header, so the first staff member is on line 2.
//! Stop at the first bad line and return its error. Use `?`.

use crate::model::{ParseError, StaffRecord};
use crate::t00_parse_line::parse_staff_line;

pub fn parse_roster(text: &str) -> Result<Vec<StaffRecord>, ParseError> {
    let mut staff = Vec::new();
    for (index, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        staff.push(parse_staff_line(index + 1, line)?);
    }
    Ok(staff)
}
