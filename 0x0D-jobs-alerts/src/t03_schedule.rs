//! Task 3: when does the job run next?
//!
//! The job runs once a day at a local time in Nairobi (for example 06:00). Nairobi is UTC+3 all
//! year: Kenya has no daylight saving time, so a fixed offset is exactly right.
//! - `nairobi()`: that offset
//! - `next_run_after(now, at)`: the first moment **strictly after** `now` when it's `at` in
//!   Nairobi, returned in UTC. If it's exactly 06:00 now, the next run is tomorrow.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::{DateTime, Days, FixedOffset, NaiveTime, TimeZone, Utc};

pub fn nairobi() -> FixedOffset {
    todo!()
}

pub fn next_run_after(now: DateTime<Utc>, at: NaiveTime) -> DateTime<Utc> {
    todo!()
}
