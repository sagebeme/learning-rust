//! Task 100 (advanced): implement `Display` so a countdown prints itself.
//!
//! `Countdown { days: 3 }` prints `"in 3 days"`, `1` → `"tomorrow"`, `0` → `"today"`,
//! `-1` → `"yesterday"`, `-4` → `"4 days ago"`. Once `Display` is implemented, `to_string()` and
//! `format!("{}", ...)` work for free.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Countdown {
    pub days: i64,
}

impl fmt::Display for Countdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.days {
            0 => write!(f, "today"),
            1 => write!(f, "tomorrow"),
            -1 => write!(f, "yesterday"),
            days if days > 1 => write!(f, "in {days} days"),
            days => write!(f, "{} days ago", days.unsigned_abs()),
        }
    }
}
