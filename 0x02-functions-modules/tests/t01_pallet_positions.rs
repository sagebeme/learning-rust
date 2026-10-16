use functions_modules::t01_pallet_positions::pallet_positions;

#[test]
fn a_two_by_three_hold() {
    assert_eq!(pallet_positions(2, 3), "A1 A2 A3\nB1 B2 B3");
}

#[test]
fn a_single_position() {
    assert_eq!(pallet_positions(1, 1), "A1");
}

#[test]
fn columns_past_nine_have_two_digits() {
    let grid = pallet_positions(1, 10);
    assert!(grid.ends_with("A9 A10"), "got {grid:?}");
}

#[test]
fn the_last_row_is_z() {
    let grid = pallet_positions(26, 1);
    assert_eq!(grid.lines().count(), 26);
    assert_eq!(grid.lines().last(), Some("Z1"));
}

#[test]
fn no_rows_is_an_empty_string() {
    assert_eq!(pallet_positions(0, 5), "");
}
