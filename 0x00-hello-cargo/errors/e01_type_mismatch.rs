// Run: rustc --edition 2024 errors/e01_type_mismatch.rs
// It won't compile. Read the whole error, then fix this file.

fn days_left() -> u32 {
    "30"
}

fn main() {
    println!("Licence expires in {} days", days_left());
}
