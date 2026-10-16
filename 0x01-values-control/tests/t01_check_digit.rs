use values_control::t01_check_digit::check_digit;

#[test]
fn sums_the_digits_and_keeps_the_last_one() {
    assert_eq!(check_digit(48213), 8);
}

#[test]
fn works_for_a_single_digit() {
    assert_eq!(check_digit(7), 7);
}

#[test]
fn works_for_zero() {
    assert_eq!(check_digit(0), 0);
}

#[test]
fn works_for_the_largest_staff_number() {
    // 4 + 2 + 9 + 4 + 9 + 6 + 7 + 2 + 9 + 5 = 57
    assert_eq!(check_digit(u32::MAX), 7);
}
