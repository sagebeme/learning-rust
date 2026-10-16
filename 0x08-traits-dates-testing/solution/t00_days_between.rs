//! Task 0: whole days between two dates.
//!
//! `days_between(1 Oct, 16 Oct)` is `15`; the other way round it's `-15`. Use chrono's
//! `NaiveDate`: a calendar date with no time and no time zone, which is exactly what a
//! qualification's dates are.

use chrono::NaiveDate;

pub fn days_between(from: NaiveDate, to: NaiveDate) -> i64 {
    (to - from).num_days()
}
