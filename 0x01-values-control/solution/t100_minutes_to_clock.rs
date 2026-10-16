//! Task 100 (advanced): minutes on duty, as a clock reading.
//!
//! `minutes_to_clock(425)` returns `"07:05"`. Hours don't wrap at 24: a 26-hour shift is
//! `"26:00"`. Both parts always have at least two digits.

pub fn minutes_to_clock(minutes: u32) -> String {
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}
