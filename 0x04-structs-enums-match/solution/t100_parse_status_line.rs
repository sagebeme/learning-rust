//! Task 100 (advanced): parse one line of a station's daily status export.
//!
//! `"48213,NBO,12"` → `Some((48213, Station::Nairobi, 12))`: staff number, station, days left.
//! Spaces around each field are fine. Anything malformed (wrong number of fields, a staff number
//! that isn't a number, an unknown station, days that aren't a whole number) gives `None`.

use crate::t00_station::Station;

pub fn parse_status_line(line: &str) -> Option<(u32, Station, i64)> {
    let mut fields = line.split(',').map(str::trim);
    let staff_number = fields.next()?.parse().ok()?;
    let station = Station::from_code(fields.next()?)?;
    let days_left = fields.next()?.parse().ok()?;
    if fields.next().is_some() {
        return None;
    }
    Some((staff_number, station, days_left))
}
