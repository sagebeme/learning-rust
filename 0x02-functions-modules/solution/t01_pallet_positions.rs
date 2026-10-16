//! Task 1: positions in a cargo hold.
//!
//! Rows are lettered from `A`, columns numbered from 1. One line per row, positions separated by
//! a single space, lines separated by `\n`, and no trailing space or newline.
//!
//! `pallet_positions(2, 3)` returns `"A1 A2 A3\nB1 B2 B3"`. `rows` is between 0 and 26.

pub fn pallet_positions(rows: u8, cols: u8) -> String {
    let mut grid = String::new();
    for row in 0..rows {
        if row > 0 {
            grid.push('\n');
        }
        let letter = char::from(b'A' + row);
        for col in 1..=cols {
            if col > 1 {
                grid.push(' ');
            }
            grid.push(letter);
            grid.push_str(&col.to_string());
        }
    }
    grid
}
