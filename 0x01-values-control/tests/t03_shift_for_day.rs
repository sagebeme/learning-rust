use values_control::t03_shift_for_day::shift_for_day;

#[test]
fn the_first_four_days() {
    assert_eq!(shift_for_day(0), "morning");
    assert_eq!(shift_for_day(1), "afternoon");
    assert_eq!(shift_for_day(2), "night");
    assert_eq!(shift_for_day(3), "off");
}

#[test]
fn the_cycle_repeats() {
    assert_eq!(shift_for_day(4), "morning");
    assert_eq!(shift_for_day(10), "night");
    assert_eq!(shift_for_day(399), "off");
}
