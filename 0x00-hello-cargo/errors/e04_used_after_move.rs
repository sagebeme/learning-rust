// Run: rustc --edition 2024 errors/e04_used_after_move.rs
// It won't compile. Read the whole error, then fix this file.
// (This one is a preview of 0x03. Fix it two different ways if you can.)

fn print_badge(name: String) {
    println!("Badge: {name}");
}

fn main() {
    let name = String::from("Amina");
    print_badge(name);
    println!("Welcome, {name}");
}
