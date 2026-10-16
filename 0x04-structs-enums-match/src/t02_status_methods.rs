//! Task 2: methods on the status.
//!
//! `is_usable`: can the person still do the work? Everything except `Expired`.
//!
//! `describe`: a short message for the app:
//! - `Valid { days_left: 90 }` → `"Valid, 90 days left"`
//! - `ExpiringSoon { days_left: 1 }` → `"Expires in 1 day"`, `ExpiringSoon { days_left: 12 }` →
//!   `"Expires in 12 days"`, `ExpiringSoon { days_left: 0 }` → `"Expires today"`
//! - `InGrace { days_overdue: 3 }` → `"Expired 3 days ago, in grace period"` (and `"1 day ago"`)
//! - `Expired` → `"Expired"`

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use crate::t01_status::QualificationStatus;

impl QualificationStatus {
    pub fn is_usable(&self) -> bool {
        todo!()
    }

    pub fn describe(&self) -> String {
        todo!()
    }
}
