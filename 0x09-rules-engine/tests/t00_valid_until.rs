mod common;

use common::{d, qualification};
use rules_engine::t00_valid_until::valid_until;

#[test]
fn adds_calendar_months() {
    let two_years = qualification("AIRSIDE", 24, 0, 30, None, &[], &[]);
    assert_eq!(valid_until(&two_years, d(2025, 3, 10)), d(2027, 3, 10));
}

#[test]
fn clamps_at_month_ends() {
    let one_month = qualification("X", 1, 0, 0, None, &[], &[]);
    assert_eq!(valid_until(&one_month, d(2026, 1, 31)), d(2026, 2, 28));
    let one_year = qualification("X", 12, 0, 0, None, &[], &[]);
    assert_eq!(valid_until(&one_year, d(2024, 2, 29)), d(2025, 2, 28));
}
