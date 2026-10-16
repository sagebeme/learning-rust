use structs_enums_match::t01_status::QualificationStatus::{self, *};

#[test]
fn only_expired_is_unusable() {
    assert!(Valid { days_left: 90 }.is_usable());
    assert!(ExpiringSoon { days_left: 3 }.is_usable());
    assert!(InGrace { days_overdue: 2 }.is_usable());
    assert!(!Expired.is_usable());
}

#[test]
fn describes_each_state() {
    let cases: [(QualificationStatus, &str); 7] = [
        (Valid { days_left: 90 }, "Valid, 90 days left"),
        (ExpiringSoon { days_left: 12 }, "Expires in 12 days"),
        (ExpiringSoon { days_left: 1 }, "Expires in 1 day"),
        (ExpiringSoon { days_left: 0 }, "Expires today"),
        (
            InGrace { days_overdue: 3 },
            "Expired 3 days ago, in grace period",
        ),
        (
            InGrace { days_overdue: 1 },
            "Expired 1 day ago, in grace period",
        ),
        (Expired, "Expired"),
    ];
    for (status, expected) in cases {
        assert_eq!(status.describe(), expected, "for {status:?}");
    }
}
