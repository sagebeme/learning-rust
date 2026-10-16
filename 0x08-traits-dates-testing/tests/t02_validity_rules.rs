use chrono::{Datelike, NaiveDate};
use proptest::prelude::*;
use traits_dates_testing::t02_validity_rules::{
    CalendarMonths, EndOfMonth, FixedDays, ValidityRule,
};

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

#[test]
fn fixed_days() {
    assert_eq!(FixedDays(90).valid_until(d(2026, 1, 1)), d(2026, 4, 1));
    assert_eq!(FixedDays(0).valid_until(d(2026, 1, 1)), d(2026, 1, 1));
    assert_eq!(FixedDays(90).describe(), "90 days");
}

#[test]
fn calendar_months() {
    assert_eq!(
        CalendarMonths(24).valid_until(d(2025, 3, 10)),
        d(2027, 3, 10)
    );
    assert_eq!(
        CalendarMonths(1).valid_until(d(2026, 1, 31)),
        d(2026, 2, 28)
    );
    assert_eq!(CalendarMonths(24).describe(), "24 months");
}

#[test]
fn end_of_month() {
    assert_eq!(EndOfMonth(24).valid_until(d(2025, 3, 10)), d(2027, 3, 31));
    assert_eq!(EndOfMonth(0).valid_until(d(2026, 2, 3)), d(2026, 2, 28));
    assert_eq!(EndOfMonth(1).valid_until(d(2028, 1, 31)), d(2028, 2, 29));
    assert_eq!(EndOfMonth(12).valid_until(d(2026, 12, 1)), d(2027, 12, 31));
}

#[test]
fn end_of_month_describes_itself_with_the_right_ordinal() {
    assert_eq!(EndOfMonth(24).describe(), "until the end of the 24th month");
    assert_eq!(EndOfMonth(1).describe(), "until the end of the 1st month");
    assert_eq!(EndOfMonth(2).describe(), "until the end of the 2nd month");
    assert_eq!(EndOfMonth(3).describe(), "until the end of the 3rd month");
    assert_eq!(EndOfMonth(11).describe(), "until the end of the 11th month");
    assert_eq!(EndOfMonth(12).describe(), "until the end of the 12th month");
    assert_eq!(EndOfMonth(21).describe(), "until the end of the 21st month");
}

// Property tests: instead of hand-picking cases, proptest tries hundreds of random ones and
// checks that a rule that must *always* hold really does.
fn any_date() -> impl Strategy<Value = NaiveDate> {
    (1990i32..2100, 1u32..=12, 1u32..=28).prop_map(|(y, m, day)| d(y, m, day))
}

proptest! {
    #[test]
    fn no_rule_ends_before_it_starts(date in any_date(), n in 0u32..600) {
        prop_assert!(FixedDays(n).valid_until(date) >= date);
        prop_assert!(CalendarMonths(n).valid_until(date) >= date);
        prop_assert!(EndOfMonth(n).valid_until(date) >= date);
    }

    #[test]
    fn end_of_month_is_always_the_last_day(date in any_date(), n in 0u32..600) {
        let until = EndOfMonth(n).valid_until(date);
        prop_assert_eq!(until.succ_opt().map(|next| next.day()), Some(1));
    }

    #[test]
    fn end_of_month_is_never_earlier_than_calendar_months(date in any_date(), n in 0u32..600) {
        prop_assert!(EndOfMonth(n).valid_until(date) >= CalendarMonths(n).valid_until(date));
    }
}
