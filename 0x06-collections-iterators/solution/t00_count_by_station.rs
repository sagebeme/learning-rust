//! Task 0: how many holdings at each station?
//!
//! Count the holdings per station. Stations with none don't appear. Use a `BTreeMap`, so the
//! result is always in the same order (the order the `Station` variants are declared in).

use std::collections::BTreeMap;

use crate::model::{Holding, Station};

pub fn count_by_station(holdings: &[Holding]) -> BTreeMap<Station, usize> {
    let mut counts = BTreeMap::new();
    for holding in holdings {
        *counts.entry(holding.station).or_insert(0) += 1;
    }
    counts
}
