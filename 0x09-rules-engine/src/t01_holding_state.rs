//! Task 1: the state of one holding on a given day.
//!
//! Work it out in this order. The first rule that applies decides:
//! 1. **Past the valid-until date?** Days overdue up to `grace_days` → `InGrace`; beyond → `Expired`.
//! 2. **Recency.** If the type has `recency_days` and more than that many days have passed since
//!    `last_used_on` (or `completed_on`, if it was never recorded) → `Lapsed`.
//! 3. **Days left** up to `warn_days` (0 counts: it's the last valid day) → `ExpiringSoon`.
//! 4. Otherwise → `Met`.
//!
//! `today` is a parameter: the engine never reads the clock.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;

use crate::model::{Holding, QualificationType, RequirementState};
use crate::t00_valid_until::valid_until;

pub fn holding_state(
    qualification: &QualificationType,
    holding: &Holding,
    today: NaiveDate,
) -> RequirementState {
    todo!()
}
