use values_control::t04_duty_board::duty_board;

#[test]
fn the_first_five_slots() {
    assert_eq!(duty_board(5), ["1", "2", "Ramp", "4", "Cargo"]);
}

#[test]
fn multiples_of_both_get_both() {
    let board = duty_board(30);
    assert_eq!(board[14], "Ramp+Cargo");
    assert_eq!(board[29], "Ramp+Cargo");
    assert_eq!(board[8], "Ramp");
    assert_eq!(board[9], "Cargo");
    assert_eq!(board[10], "11");
}

#[test]
fn has_one_entry_per_slot() {
    assert_eq!(duty_board(100).len(), 100);
}

#[test]
fn zero_slots_is_an_empty_board() {
    assert!(duty_board(0).is_empty());
}
