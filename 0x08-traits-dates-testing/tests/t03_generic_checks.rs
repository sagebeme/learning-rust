use chrono::NaiveDate;
use traits_dates_testing::t02_validity_rules::{
    CalendarMonths, EndOfMonth, FixedDays, ValidityRule,
};
use traits_dates_testing::t03_generic_checks::{is_valid_on, strictest};

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

#[test]
fn valid_from_completion_to_the_last_day_inclusive() {
    let rule = FixedDays(10);
    let completed = d(2026, 10, 1);
    assert!(!is_valid_on(&rule, completed, d(2026, 9, 30)));
    assert!(is_valid_on(&rule, completed, d(2026, 10, 1)));
    assert!(is_valid_on(&rule, completed, d(2026, 10, 11)));
    assert!(!is_valid_on(&rule, completed, d(2026, 10, 12)));
}

#[test]
fn works_with_any_rule_type_including_trait_objects() {
    let completed = d(2025, 3, 10);
    let today = d(2027, 3, 20);
    assert!(!is_valid_on(&CalendarMonths(24), completed, today));
    assert!(is_valid_on(&EndOfMonth(24), completed, today));

    let boxed: Box<dyn ValidityRule> = Box::new(EndOfMonth(24));
    assert!(is_valid_on(boxed.as_ref(), completed, today));
}

#[test]
fn the_strictest_rule_wins() {
    let rules: Vec<Box<dyn ValidityRule>> = vec![
        Box::new(EndOfMonth(24)),
        Box::new(CalendarMonths(24)),
        Box::new(FixedDays(800)),
    ];
    assert_eq!(strictest(&rules, d(2025, 3, 10)), Some(d(2027, 3, 10)));
}

#[test]
fn no_rules_no_answer() {
    assert_eq!(strictest(&[], d(2025, 3, 10)), None);
}
