// Run: rustc --edition 2024 errors/e03_stray_semicolon.rs
// It won't compile. Read the whole error, then fix this file.

fn overtime_hours(hours: u32) -> u32 {
    if hours > 40 { hours - 40 } else { 0 };
}

fn main() {
    println!("Overtime: {} hours", overtime_hours(45));
}
