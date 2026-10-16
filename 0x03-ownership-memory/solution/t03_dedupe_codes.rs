//! Task 3: remove duplicate qualification codes, in place.
//!
//! Keep the first time each code appears and drop later repeats, keeping the original order.
//! `["FIRE", "DG", "FIRE", "AVSEC", "DG"]` becomes `["FIRE", "DG", "AVSEC"]`.

use std::collections::HashSet;

pub fn dedupe_codes(codes: &mut Vec<String>) {
    let mut seen = HashSet::new();
    codes.retain(|code| seen.insert(code.clone()));
}
