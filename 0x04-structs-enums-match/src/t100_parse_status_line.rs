//! Task 100 (advanced): parse one line of a station's daily status export.
//!
//! `"48213,NBO,12"` → `Some((48213, Station::Nairobi, 12))`: staff number, station, days left.
//! Spaces around each field are fine. Anything malformed (wrong number of fields, a staff number
//! that isn't a number, an unknown station, days that aren't a whole number) gives `None`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use crate::t00_station::Station;

pub fn parse_status_line(line: &str) -> Option<(u32, Station, i64)> {
    todo!()
}
