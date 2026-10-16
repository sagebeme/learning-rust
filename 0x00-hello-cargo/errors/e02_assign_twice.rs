// Run: rustc --edition 2024 errors/e02_assign_twice.rs
// It won't compile. Read the whole error, then fix this file.

fn main() {
    let total = 0;
    for hours in [8, 8, 6] {
        total += hours;
    }
    println!("Hours this week: {total}");
}
