//! Task 4: feed the alert job from the engine.
//!
//! The job's input (`ExpiringItem`: who, which code, valid until when) comes from the engine. For
//! every person and every holding:
//! - look up the holding's qualification type in the catalogue; skip codes that aren't in it (a
//!   real job would also log them)
//! - work out `valid_until` with `twa_engine::t00_valid_until::valid_until`
//! - if someone holds the same code more than once, keep only the **latest** `valid_until`: a
//!   renewed certificate must not trigger reminders about the old one
//!
//! Return the items sorted by staff number, then code.

use std::collections::BTreeMap;

use twa_engine::model::{Catalogue, Holding};
use twa_engine::t00_valid_until::valid_until;

use crate::model::ExpiringItem;

pub fn expiring_items(
    catalogue: &Catalogue,
    holdings: &BTreeMap<u32, Vec<Holding>>,
) -> Vec<ExpiringItem> {
    // (staff number, code) → the latest valid-until date. A BTreeMap keeps the output sorted.
    let mut latest = BTreeMap::new();
    for (&staff_number, held) in holdings {
        for holding in held {
            let Some(qualification) = catalogue.get(&holding.code) else {
                continue;
            };
            let until = valid_until(qualification, holding.completed_on);
            latest
                .entry((staff_number, holding.code.clone()))
                .and_modify(|current| {
                    if until > *current {
                        *current = until;
                    }
                })
                .or_insert(until);
        }
    }
    latest
        .into_iter()
        .map(|((staff_number, code), valid_until)| ExpiringItem {
            staff_number,
            code,
            valid_until,
        })
        .collect()
}
