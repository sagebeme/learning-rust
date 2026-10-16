use chrono::NaiveDate;
use traits_dates_testing::t01_add_months::add_months;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

// A table-driven test: each row is one case. Adding a case is one line, and a failure says which.
#[test]
fn adds_calendar_months() {
    let cases = [
        (d(2026, 3, 15), 12, d(2027, 3, 15)),
        (d(2026, 3, 15), 0, d(2026, 3, 15)),
        (d(2026, 11, 30), 3, d(2027, 2, 28)),
        (d(2026, 1, 31), 1, d(2026, 2, 28)),
        (d(2028, 1, 31), 1, d(2028, 2, 29)),
        (d(2024, 2, 29), 12, d(2025, 2, 28)),
        (d(2024, 2, 29), 48, d(2028, 2, 29)),
        (d(2026, 8, 31), 1, d(2026, 9, 30)),
    ];
    for (start, months, expected) in cases {
        assert_eq!(
            add_months(start, months),
            expected,
            "{start} + {months} months"
        );
    }
}

#[test]
fn stops_at_the_maximum_date() {
    assert_eq!(add_months(NaiveDate::MAX, 1), NaiveDate::MAX);
}
