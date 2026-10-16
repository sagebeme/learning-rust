//! Task 1: what expires in the next N days?
//!
//! Return the holdings with 0 to `days` days left (inclusive), soonest first. Holdings with the
//! same days left are ordered by staff number. Already-expired holdings (negative days) are not
//! included. Return references into the input: don't copy the holdings.

use crate::model::Holding;

pub fn expiring_within(holdings: &[Holding], days: i64) -> Vec<&Holding> {
    let mut soon: Vec<&Holding> = holdings
        .iter()
        .filter(|h| (0..=days).contains(&h.days_left))
        .collect();
    soon.sort_by_key(|h| (h.days_left, h.staff_number));
    soon
}
