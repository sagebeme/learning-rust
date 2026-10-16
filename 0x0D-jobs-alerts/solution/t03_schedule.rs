//! Task 3: when does the job run next?
//!
//! The job runs once a day at a local time in Nairobi (for example 06:00). Nairobi is UTC+3 all
//! year: Kenya has no daylight saving time, so a fixed offset is exactly right.
//! - `nairobi()`: that offset
//! - `next_run_after(now, at)`: the first moment **strictly after** `now` when it's `at` in
//!   Nairobi, returned in UTC. If it's exactly 06:00 now, the next run is tomorrow.

use chrono::{DateTime, FixedOffset, NaiveTime, TimeDelta, Utc};

pub fn nairobi() -> FixedOffset {
    FixedOffset::east_opt(3 * 60 * 60).expect("UTC+3 is a valid offset")
}

pub fn next_run_after(now: DateTime<Utc>, at: NaiveTime) -> DateTime<Utc> {
    let zone = nairobi();
    let local_now = now.with_timezone(&zone);
    let today_at = local_now
        .date_naive()
        .and_time(at)
        .and_local_timezone(zone)
        .single()
        .expect("a fixed offset has exactly one local time for every instant");
    // No daylight saving in Nairobi, so "tomorrow at the same time" is exactly 24 hours later.
    let next = if today_at > local_now {
        today_at
    } else {
        today_at + TimeDelta::days(1)
    };
    next.with_timezone(&Utc)
}
