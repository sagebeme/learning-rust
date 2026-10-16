use chrono::NaiveDate;
use traits_dates_testing::t00_days_between::days_between;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

#[test]
fn counts_forwards_and_backwards() {
    assert_eq!(days_between(d(2026, 10, 1), d(2026, 10, 16)), 15);
    assert_eq!(days_between(d(2026, 10, 16), d(2026, 10, 1)), -15);
    assert_eq!(days_between(d(2026, 10, 16), d(2026, 10, 16)), 0);
}

#[test]
fn knows_about_leap_years() {
    assert_eq!(days_between(d(2024, 2, 28), d(2024, 3, 1)), 2);
    assert_eq!(days_between(d(2025, 2, 28), d(2025, 3, 1)), 1);
    assert_eq!(days_between(d(2025, 1, 1), d(2026, 1, 1)), 365);
}
