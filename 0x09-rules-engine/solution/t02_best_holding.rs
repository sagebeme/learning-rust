//! Task 2: the best holding for a requirement.
//!
//! A requirement for `code` can be satisfied by a holding of `code` itself, or of any code in its
//! `equivalents`. Work out each candidate holding's state **using the holding's own qualification
//! type**, and pick the best one:
//! 1. the lowest `rank()`
//! 2. then the latest valid-until date
//! 3. then the alphabetically first code
//!
//! Return the state and the code of the holding used, or `(Missing, None)` if there's none.
//! Errors: `UnknownQualification` if `code`, or the code of a candidate holding, isn't in the
//! catalogue.

use std::cmp::Reverse;

use chrono::NaiveDate;

use crate::model::{Catalogue, EngineError, Holding, RequirementState};
use crate::t00_valid_until::valid_until;
use crate::t01_holding_state::holding_state;

pub fn best_holding(
    catalogue: &Catalogue,
    code: &str,
    holdings: &[Holding],
    today: NaiveDate,
) -> Result<(RequirementState, Option<String>), EngineError> {
    let required = catalogue
        .get(code)
        .ok_or_else(|| EngineError::UnknownQualification(code.to_string()))?;
    let accepts =
        |candidate: &str| candidate == code || required.equivalents.iter().any(|e| e == candidate);

    let mut best: Option<((u8, Reverse<NaiveDate>, String), RequirementState)> = None;
    for holding in holdings.iter().filter(|h| accepts(&h.code)) {
        let own_type = catalogue
            .get(&holding.code)
            .ok_or_else(|| EngineError::UnknownQualification(holding.code.clone()))?;
        let state = holding_state(own_type, holding, today);
        let key = (
            state.rank(),
            Reverse(valid_until(own_type, holding.completed_on)),
            holding.code.clone(),
        );
        if best.as_ref().is_none_or(|(best_key, _)| key < *best_key) {
            best = Some((key, state));
        }
    }

    Ok(match best {
        Some(((_, _, code), state)) => (state, Some(code)),
        None => (RequirementState::Missing, None),
    })
}
