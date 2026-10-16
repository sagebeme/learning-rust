//! Task 0: how many holdings at each station?
//!
//! Count the holdings per station. Stations with none don't appear. Use a `BTreeMap`, so the
//! result is always in the same order (the order the `Station` variants are declared in).

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::BTreeMap;

use crate::model::{Holding, Station};

pub fn count_by_station(holdings: &[Holding]) -> BTreeMap<Station, usize> {
    todo!()
}
