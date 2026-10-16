//! Task 2: split a full name into first name and the rest, without copying.
//!
//! `split_name("Amina Wanjiru Otieno")` returns `("Amina", "Wanjiru Otieno")`. Ignore spaces at
//! either end. One name gives `("Kip", "")`; an empty string gives `("", "")`. Both parts are
//! slices of the text you were given.

pub fn split_name(full_name: &str) -> (&str, &str) {
    let trimmed = full_name.trim();
    match trimmed.split_once(char::is_whitespace) {
        Some((first, rest)) => (first, rest.trim_start()),
        None => (trimmed, ""),
    }
}
