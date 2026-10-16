use functions_modules::t02_weekly_pay::weekly_pay_cents;

#[test]
fn a_standard_week_has_no_overtime() {
    assert_eq!(weekly_pay_cents(40, 50_000), 2_000_000);
}

#[test]
fn hours_past_forty_are_paid_at_one_and_a_half() {
    // 40 * 500 + 5 * 750 = 23,750 shillings
    assert_eq!(weekly_pay_cents(45, 50_000), 2_375_000);
}

#[test]
fn a_part_time_week() {
    assert_eq!(weekly_pay_cents(12, 50_000), 600_000);
}

#[test]
fn half_cents_are_rounded_down() {
    // An odd rate: 1 overtime hour at 1.5 * 333 = 499.5 cents, which rounds down to 499.
    assert_eq!(weekly_pay_cents(41, 333), 40 * 333 + 499);
}

#[test]
fn no_hours_no_pay() {
    assert_eq!(weekly_pay_cents(0, 50_000), 0);
}
