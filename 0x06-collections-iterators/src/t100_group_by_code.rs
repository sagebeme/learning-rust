//! Task 100 (advanced): who holds each qualification?
//!
//! Map each qualification code to the staff numbers holding it, sorted and without repeats.
//! The keys borrow from the holdings: `&str`, not `String`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::BTreeMap;

use crate::model::Holding;

pub fn group_by_code(holdings: &[Holding]) -> BTreeMap<&str, Vec<u32>> {
    todo!()
}
