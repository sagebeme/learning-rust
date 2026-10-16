use values_control::t00_expiry_label::expiry_label;

#[test]
fn negative_days_means_expired() {
    assert_eq!(expiry_label(-1), "expired");
    assert_eq!(expiry_label(-400), "expired");
}

#[test]
fn zero_days_means_today() {
    assert_eq!(expiry_label(0), "expires today");
}

#[test]
fn up_to_thirty_days_is_expiring_soon() {
    assert_eq!(expiry_label(1), "expiring soon");
    assert_eq!(expiry_label(30), "expiring soon");
}

#[test]
fn more_than_thirty_days_is_valid() {
    assert_eq!(expiry_label(31), "valid");
    assert_eq!(expiry_label(730), "valid");
}
