//! Task 2: three ways a qualification's validity is defined, behind one trait.
//!
//! `valid_until(completed_on)` is the **last day** the qualification is valid:
//! - `FixedDays(n)`: `n` days after completion. `FixedDays(90)` from 1 Jan is 1 April.
//! - `CalendarMonths(n)`: `n` calendar months after completion, using `add_months` from task 1.
//! - `EndOfMonth(n)`: the last day of the month, `n` months after the month of completion. Common
//!   in aviation: completed 10 March 2025, `EndOfMonth(24)` is valid until 31 March 2027.
//!
//! `describe()` returns `"90 days"`, `"24 months"` or `"until the end of the 24th month"`.

use chrono::{Datelike, Days, NaiveDate};

use crate::t01_add_months::add_months;

pub trait ValidityRule {
    fn valid_until(&self, completed_on: NaiveDate) -> NaiveDate;
    fn describe(&self) -> String;
}

pub struct FixedDays(pub u32);
pub struct CalendarMonths(pub u32);
pub struct EndOfMonth(pub u32);

impl ValidityRule for FixedDays {
    fn valid_until(&self, completed_on: NaiveDate) -> NaiveDate {
        completed_on
            .checked_add_days(Days::new(u64::from(self.0)))
            .unwrap_or(NaiveDate::MAX)
    }

    fn describe(&self) -> String {
        format!("{} days", self.0)
    }
}

impl ValidityRule for CalendarMonths {
    fn valid_until(&self, completed_on: NaiveDate) -> NaiveDate {
        add_months(completed_on, self.0)
    }

    fn describe(&self) -> String {
        format!("{} months", self.0)
    }
}

impl ValidityRule for EndOfMonth {
    fn valid_until(&self, completed_on: NaiveDate) -> NaiveDate {
        // First day of the completion month, plus n + 1 months, minus one day: the last day of
        // the month n months later.
        let first_of_month = completed_on.with_day(1).unwrap_or(completed_on);
        add_months(first_of_month, self.0.saturating_add(1))
            .pred_opt()
            .unwrap_or(NaiveDate::MAX)
    }

    fn describe(&self) -> String {
        format!("until the end of the {} month", ordinal(self.0))
    }
}

fn ordinal(n: u32) -> String {
    let suffix = match (n % 10, n % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}
