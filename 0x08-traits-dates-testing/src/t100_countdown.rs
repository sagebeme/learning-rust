//! Task 100 (advanced): implement `Display` so a countdown prints itself.
//!
//! `Countdown { days: 3 }` prints `"in 3 days"`, `1` → `"tomorrow"`, `0` → `"today"`,
//! `-1` → `"yesterday"`, `-4` → `"4 days ago"`. Once `Display` is implemented, `to_string()` and
//! `format!("{}", ...)` work for free.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Countdown {
    pub days: i64,
}

impl fmt::Display for Countdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
