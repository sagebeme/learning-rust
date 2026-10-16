//! Task 100 (advanced): a stack of pallets, drawn right-aligned.
//!
//! `stack_pallets(3)` returns `"  #\n ##\n###"`. Zero gives an empty string.

pub fn stack_pallets(height: usize) -> String {
    let mut lines = Vec::with_capacity(height);
    for width in 1..=height {
        lines.push(format!(
            "{}{}",
            " ".repeat(height - width),
            "#".repeat(width)
        ));
    }
    lines.join("\n")
}
