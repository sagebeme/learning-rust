//! Task 100 (advanced): who holds each qualification?
//!
//! Map each qualification code to the staff numbers holding it, sorted and without repeats.
//! The keys borrow from the holdings: `&str`, not `String`.

use std::collections::BTreeMap;

use crate::model::Holding;

pub fn group_by_code(holdings: &[Holding]) -> BTreeMap<&str, Vec<u32>> {
    let mut groups: BTreeMap<&str, Vec<u32>> = BTreeMap::new();
    for holding in holdings {
        groups
            .entry(holding.code.as_str())
            .or_default()
            .push(holding.staff_number);
    }
    for staff in groups.values_mut() {
        staff.sort_unstable();
        staff.dedup();
    }
    groups
}
