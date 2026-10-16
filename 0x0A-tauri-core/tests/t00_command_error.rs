use serde_json::json;
use tauri_core::t00_command_error::CommandError;

#[test]
fn each_error_has_a_kind() {
    assert_eq!(CommandError::NotFound("x".into()).kind(), "notFound");
    assert_eq!(
        CommandError::Invalid {
            field: "code".into(),
            reason: "bad".into()
        }
        .kind(),
        "invalid"
    );
    assert_eq!(CommandError::Busy.kind(), "busy");
    assert_eq!(CommandError::Internal("x".into()).kind(), "internal");
}

#[test]
fn serializes_as_kind_and_message() {
    let error = CommandError::NotFound("staff member 48213".into());
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({ "kind": "notFound", "message": "staff member 48213 was not found" })
    );
}

#[test]
fn an_invalid_field_says_which_field() {
    let error = CommandError::Invalid {
        field: "completedOn".into(),
        reason: "can't be in the future".into(),
    };
    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({ "kind": "invalid", "message": "completedOn: can't be in the future" })
    );
}
