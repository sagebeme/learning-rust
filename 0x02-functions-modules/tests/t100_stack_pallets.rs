use functions_modules::t100_stack_pallets::stack_pallets;

#[test]
fn a_stack_of_three() {
    assert_eq!(stack_pallets(3), "  #\n ##\n###");
}

#[test]
fn a_stack_of_one() {
    assert_eq!(stack_pallets(1), "#");
}

#[test]
fn every_line_is_as_wide_as_the_stack_is_tall() {
    let stack = stack_pallets(12);
    assert_eq!(stack.lines().count(), 12);
    assert!(stack.lines().all(|line| line.len() == 12));
}

#[test]
fn zero_is_empty() {
    assert_eq!(stack_pallets(0), "");
}
