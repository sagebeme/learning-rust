//! Task 0: parse one roster line, or say exactly what's wrong with it.
//!
//! A line is `staff_number,name,station`, for example `48213,Amina Wanjiru,NBO`. Trim each field.
//! Check, in this order, and return the first problem as a `ParseError`:
//! 1. exactly 3 fields (`WrongFieldCount`)
//! 2. the staff number is a whole number (`BadStaffNumber`, with the trimmed text)
//! 3. the name isn't empty (`EmptyName`)
//! 4. the station is one of `STATIONS`, in capitals (`UnknownStation`, with the trimmed text)

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use crate::model::{ParseError, STATIONS, StaffRecord};

pub fn parse_staff_line(line_number: usize, line: &str) -> Result<StaffRecord, ParseError> {
    todo!()
}
