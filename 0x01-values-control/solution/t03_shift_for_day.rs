//! Task 3: the four-day shift cycle.
//!
//! Day 0 is `"morning"`, day 1 `"afternoon"`, day 2 `"night"`, day 3 `"off"`, and then the
//! cycle repeats: day 4 is `"morning"` again. Use `match`.

pub fn shift_for_day(day: u32) -> &'static str {
    match day % 4 {
        0 => "morning",
        1 => "afternoon",
        2 => "night",
        _ => "off",
    }
}
