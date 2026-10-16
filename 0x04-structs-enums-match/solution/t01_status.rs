//! Task 1: what state is a qualification in?
//!
//! Given the days left until it expires, how many days of warning to give, and how many days of
//! grace after expiry:
//!
//! - more than `warn_days` left: `Valid { days_left }`
//! - 0 to `warn_days` left: `ExpiringSoon { days_left }` (0 means it expires today, still valid)
//! - expired, but no more than `grace_days` ago: `InGrace { days_overdue }`
//! - expired longer ago than that: `Expired`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualificationStatus {
    Valid { days_left: i64 },
    ExpiringSoon { days_left: i64 },
    InGrace { days_overdue: i64 },
    Expired,
}

pub fn status(days_left: i64, warn_days: i64, grace_days: i64) -> QualificationStatus {
    if days_left > warn_days {
        QualificationStatus::Valid { days_left }
    } else if days_left >= 0 {
        QualificationStatus::ExpiringSoon { days_left }
    } else if -days_left <= grace_days {
        QualificationStatus::InGrace {
            days_overdue: -days_left,
        }
    } else {
        QualificationStatus::Expired
    }
}
