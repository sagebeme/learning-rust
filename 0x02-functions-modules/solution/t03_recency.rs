//! Task 3: recency and refreshers.
//!
//! `is_current`: someone who last used a qualification `days_since_last_used` days ago is still
//! current if that's no more than `recency_limit_days`.
//!
//! `refresher_due_in`: how many days until a refresher is due. Negative means it's overdue.
//! `refresher_due_in(400, 365)` is `-35`.

pub fn is_current(days_since_last_used: u32, recency_limit_days: u32) -> bool {
    days_since_last_used <= recency_limit_days
}

pub fn refresher_due_in(days_since_training: u32, refresher_interval_days: u32) -> i64 {
    i64::from(refresher_interval_days) - i64::from(days_since_training)
}
