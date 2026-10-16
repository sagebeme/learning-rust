//! Task 1: parse a whole roster.
//!
//! The first line is a header (`staff_number,name,station`): skip it. Skip blank lines too.
//! Line numbers count from 1 and include the header, so the first staff member is on line 2.
//! Stop at the first bad line and return its error. Use `?`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use crate::model::{ParseError, StaffRecord};
use crate::t00_parse_line::parse_staff_line;

pub fn parse_roster(text: &str) -> Result<Vec<StaffRecord>, ParseError> {
    todo!()
}
