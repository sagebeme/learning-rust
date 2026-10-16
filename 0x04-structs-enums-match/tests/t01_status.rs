use structs_enums_match::t01_status::{QualificationStatus, status};

// 30 days' warning, 14 days' grace.
fn s(days_left: i64) -> QualificationStatus {
    status(days_left, 30, 14)
}

#[test]
fn valid_with_plenty_of_time_left() {
    assert_eq!(s(31), QualificationStatus::Valid { days_left: 31 });
    assert_eq!(s(400), QualificationStatus::Valid { days_left: 400 });
}

#[test]
fn expiring_soon_inside_the_warning_window() {
    assert_eq!(s(30), QualificationStatus::ExpiringSoon { days_left: 30 });
    assert_eq!(s(1), QualificationStatus::ExpiringSoon { days_left: 1 });
}

#[test]
fn expiring_today_is_still_valid_today() {
    assert_eq!(s(0), QualificationStatus::ExpiringSoon { days_left: 0 });
}

#[test]
fn in_grace_just_after_expiry() {
    assert_eq!(s(-1), QualificationStatus::InGrace { days_overdue: 1 });
    assert_eq!(s(-14), QualificationStatus::InGrace { days_overdue: 14 });
}

#[test]
fn expired_after_the_grace_period() {
    assert_eq!(s(-15), QualificationStatus::Expired);
}

#[test]
fn no_grace_period_means_expired_the_day_after() {
    assert_eq!(status(-1, 30, 0), QualificationStatus::Expired);
}
