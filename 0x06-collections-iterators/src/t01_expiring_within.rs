//! Task 1: what expires in the next N days?
//!
//! Return the holdings with 0 to `days` days left (inclusive), soonest first. Holdings with the
//! same days left are ordered by staff number. Already-expired holdings (negative days) are not
//! included. Return references into the input: don't copy the holdings.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use crate::model::Holding;

pub fn expiring_within(holdings: &[Holding], days: i64) -> Vec<&Holding> {
    todo!()
}
