use structs_enums_match::t03_qualification_type::{QualificationType, TypeError};

#[test]
fn builds_a_valid_type() {
    let dg = QualificationType::new("Dangerous Goods", 730, 30, Some(180)).unwrap();
    assert_eq!(dg.name(), "Dangerous Goods");
    assert_eq!(dg.validity_days(), 730);
    assert_eq!(dg.grace_days(), 30);
    assert_eq!(dg.recency_days(), Some(180));
}

#[test]
fn trims_the_name() {
    let fire = QualificationType::new("  Fire Safety ", 365, 0, None).unwrap();
    assert_eq!(fire.name(), "Fire Safety");
}

#[test]
fn rejects_a_blank_name() {
    assert_eq!(
        QualificationType::new("", 365, 0, None),
        Err(TypeError::EmptyName)
    );
    assert_eq!(
        QualificationType::new("   ", 365, 0, None),
        Err(TypeError::EmptyName)
    );
}

#[test]
fn rejects_zero_validity() {
    assert_eq!(
        QualificationType::new("Ramp", 0, 0, None),
        Err(TypeError::ZeroValidity)
    );
}

#[test]
fn rejects_grace_as_long_as_the_validity() {
    assert_eq!(
        QualificationType::new("Ramp", 30, 30, None),
        Err(TypeError::GraceTooLong)
    );
    assert_eq!(
        QualificationType::new("Ramp", 30, 31, None),
        Err(TypeError::GraceTooLong)
    );
    assert!(QualificationType::new("Ramp", 30, 29, None).is_ok());
}
