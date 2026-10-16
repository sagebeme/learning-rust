use values_control::t02_total_hours::total_hours;

#[test]
fn adds_up_a_week() {
    assert_eq!(total_hours(&[8, 8, 8, 10, 6]), Some(40));
}

#[test]
fn an_empty_list_is_zero_hours() {
    assert_eq!(total_hours(&[]), Some(0));
}

#[test]
fn the_largest_total_that_fits_is_fine() {
    assert_eq!(total_hours(&[65_000, 535]), Some(65_535));
}

#[test]
fn a_total_that_does_not_fit_is_none_not_a_wrong_number() {
    assert_eq!(total_hours(&[65_000, 536]), None);
    assert_eq!(total_hours(&[u16::MAX, 1, 1]), None);
}
