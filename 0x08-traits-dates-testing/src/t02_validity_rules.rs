//! Task 2: three ways a qualification's validity is defined, behind one trait.
//!
//! `valid_until(completed_on)` is the **last day** the qualification is valid:
//! - `FixedDays(n)`: `n` days after completion. `FixedDays(90)` from 1 Jan is 1 April.
//! - `CalendarMonths(n)`: `n` calendar months after completion, using `add_months` from task 1.
//! - `EndOfMonth(n)`: the last day of the month, `n` months after the month of completion. Common
//!   in aviation: completed 10 March 2025, `EndOfMonth(24)` is valid until 31 March 2027.
//!
//! `describe()` returns `"90 days"`, `"24 months"` or `"until the end of the 24th month"`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

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
        todo!()
    }

    fn describe(&self) -> String {
        todo!()
    }
}

impl ValidityRule for CalendarMonths {
    fn valid_until(&self, completed_on: NaiveDate) -> NaiveDate {
        todo!()
    }

    fn describe(&self) -> String {
        todo!()
    }
}

impl ValidityRule for EndOfMonth {
    fn valid_until(&self, completed_on: NaiveDate) -> NaiveDate {
        todo!()
    }

    fn describe(&self) -> String {
        todo!()
    }
}
