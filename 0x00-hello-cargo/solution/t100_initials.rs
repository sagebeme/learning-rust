//! Task 100 (advanced): initials for a name badge.
//!
//! `initials("Amina Wanjiru Otieno")` returns `"A.W.O."`. Extra spaces don't matter,
//! lower-case names come out upper-case, and an empty name gives an empty string.

pub fn initials(full_name: &str) -> String {
    let mut result = String::new();
    for word in full_name.split_whitespace() {
        if let Some(first) = word.chars().next() {
            result.extend(first.to_uppercase());
            result.push('.');
        }
    }
    result
}
