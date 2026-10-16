//! Task 0: is this a valid station code?
//!
//! A station code is exactly three upper-case English letters: `"NBO"` yes; `"nbo"`, `"NB"`,
//! `"NBOX"`, `"N8O"` no.

pub fn is_station_code(code: &str) -> bool {
    code.len() == 3 && code.bytes().all(|b| b.is_ascii_uppercase())
}
