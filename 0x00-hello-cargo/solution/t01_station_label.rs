//! Task 1: station labels.
//!
//! `station_label("NBO", "Nairobi")` returns `"NBO (Nairobi)"`.

pub fn station_label(code: &str, city: &str) -> String {
    format!("{code} ({city})")
}
