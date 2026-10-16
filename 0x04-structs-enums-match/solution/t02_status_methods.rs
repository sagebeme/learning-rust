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

use crate::t01_status::QualificationStatus;

fn days(count: i64) -> String {
    if count == 1 {
        "1 day".to_string()
    } else {
        format!("{count} days")
    }
}

impl QualificationStatus {
    pub fn is_usable(&self) -> bool {
        !matches!(self, QualificationStatus::Expired)
    }

    pub fn describe(&self) -> String {
        match *self {
            QualificationStatus::Valid { days_left } => format!("Valid, {} left", days(days_left)),
            QualificationStatus::ExpiringSoon { days_left: 0 } => "Expires today".to_string(),
            QualificationStatus::ExpiringSoon { days_left } => {
                format!("Expires in {}", days(days_left))
            }
            QualificationStatus::InGrace { days_overdue } => {
                format!("Expired {} ago, in grace period", days(days_overdue))
            }
            QualificationStatus::Expired => "Expired".to_string(),
        }
    }
}
