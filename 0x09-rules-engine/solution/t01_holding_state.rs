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

use chrono::NaiveDate;

use crate::model::{Holding, QualificationType, RequirementState};
use crate::t00_valid_until::valid_until;

pub fn holding_state(
    qualification: &QualificationType,
    holding: &Holding,
    today: NaiveDate,
) -> RequirementState {
    let valid_until = valid_until(qualification, holding.completed_on);

    if today > valid_until {
        let days_overdue = (today - valid_until).num_days();
        return if days_overdue <= i64::from(qualification.grace_days) {
            RequirementState::InGrace {
                valid_until,
                days_overdue,
            }
        } else {
            RequirementState::Expired { valid_until }
        };
    }

    if let Some(recency_days) = qualification.recency_days {
        let last_used_on = holding.last_used_on.unwrap_or(holding.completed_on);
        if (today - last_used_on).num_days() > i64::from(recency_days) {
            return RequirementState::Lapsed { last_used_on };
        }
    }

    let days_left = (valid_until - today).num_days();
    if days_left <= i64::from(qualification.warn_days) {
        RequirementState::ExpiringSoon {
            valid_until,
            days_left,
        }
    } else {
        RequirementState::Met { valid_until }
    }
}
