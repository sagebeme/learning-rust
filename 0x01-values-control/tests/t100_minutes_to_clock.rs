use values_control::t100_minutes_to_clock::minutes_to_clock;

#[test]
fn pads_hours_and_minutes() {
    assert_eq!(minutes_to_clock(425), "07:05");
}

#[test]
fn zero_minutes() {
    assert_eq!(minutes_to_clock(0), "00:00");
}

#[test]
fn does_not_wrap_at_24_hours() {
    assert_eq!(minutes_to_clock(26 * 60), "26:00");
}

#[test]
fn long_shifts_keep_all_their_hour_digits() {
    assert_eq!(minutes_to_clock(100 * 60 + 59), "100:59");
}
