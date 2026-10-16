//! Task 1: tidy a name in place.
//!
//! Change the `String` you're given: trim spaces from both ends and squeeze every run of spaces
//! inside it down to one. `"  Amina   Wanjiru "` becomes `"Amina Wanjiru"`. Return nothing: the
//! caller's `String` is the result.

pub fn tidy_name(name: &mut String) {
    let tidy = name.split_whitespace().collect::<Vec<_>>().join(" ");
    *name = tidy;
}
