use functions_modules::t03_recency::{is_current, refresher_due_in};

#[test]
fn current_up_to_and_including_the_limit() {
    assert!(is_current(0, 90));
    assert!(is_current(90, 90));
}

#[test]
fn not_current_past_the_limit() {
    assert!(!is_current(91, 90));
}

#[test]
fn refresher_due_in_the_future() {
    assert_eq!(refresher_due_in(300, 365), 65);
}

#[test]
fn refresher_due_today() {
    assert_eq!(refresher_due_in(365, 365), 0);
}

#[test]
fn refresher_overdue_is_negative() {
    assert_eq!(refresher_due_in(400, 365), -35);
}

#[test]
fn handles_the_extremes_without_overflowing() {
    assert_eq!(refresher_due_in(u32::MAX, 0), -i64::from(u32::MAX));
    assert_eq!(refresher_due_in(0, u32::MAX), i64::from(u32::MAX));
}
