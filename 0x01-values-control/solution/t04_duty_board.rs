//! Task 4: the duty board.
//!
//! For each slot from 1 to `slots`: multiples of 3 say `"Ramp"`, multiples of 5 say `"Cargo"`,
//! multiples of both say `"Ramp+Cargo"`, and every other slot shows its number.
//!
//! `duty_board(5)` returns `["1", "2", "Ramp", "4", "Cargo"]`.

pub fn duty_board(slots: u32) -> Vec<String> {
    let mut board = Vec::new();
    for slot in 1..=slots {
        let entry = match (slot % 3, slot % 5) {
            (0, 0) => "Ramp+Cargo".to_string(),
            (0, _) => "Ramp".to_string(),
            (_, 0) => "Cargo".to_string(),
            _ => slot.to_string(),
        };
        board.push(entry);
    }
    board
}
