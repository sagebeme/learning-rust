//! Task 0: parse one roster line, or say exactly what's wrong with it.
//!
//! A line is `staff_number,name,station`, for example `48213,Amina Wanjiru,NBO`. Trim each field.
//! Check, in this order, and return the first problem as a `ParseError`:
//! 1. exactly 3 fields (`WrongFieldCount`)
//! 2. the staff number is a whole number (`BadStaffNumber`, with the trimmed text)
//! 3. the name isn't empty (`EmptyName`)
//! 4. the station is one of `STATIONS`, in capitals (`UnknownStation`, with the trimmed text)

use crate::model::{ParseError, STATIONS, StaffRecord};

pub fn parse_staff_line(line_number: usize, line: &str) -> Result<StaffRecord, ParseError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    let &[number, name, station] = fields.as_slice() else {
        return Err(ParseError::WrongFieldCount {
            line: line_number,
            found: fields.len(),
        });
    };

    let staff_number = number.parse().map_err(|_| ParseError::BadStaffNumber {
        line: line_number,
        value: number.to_string(),
    })?;

    if name.is_empty() {
        return Err(ParseError::EmptyName { line: line_number });
    }

    if !STATIONS.contains(&station) {
        return Err(ParseError::UnknownStation {
            line: line_number,
            code: station.to_string(),
        });
    }

    Ok(StaffRecord {
        staff_number,
        name: name.to_string(),
        station: station.to_string(),
    })
}
