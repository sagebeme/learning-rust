use tauri_core::t00_command_error::CommandError;
use tauri_core::t03_capabilities::{Problem, check_capability};

const COMMANDS: [&str; 2] = ["add_holding", "staff_summary"];

#[test]
fn a_tidy_capability_has_no_problems() {
    let json = r#"{"identifier": "main", "windows": ["main"],
                   "permissions": ["core:default", "allow-add-holding", "allow-staff-summary"]}"#;
    assert_eq!(check_capability(json, &COMMANDS), Ok(vec![]));
}

#[test]
fn finds_a_command_without_permission() {
    let json =
        r#"{"identifier": "main", "windows": ["main"], "permissions": ["allow-add-holding"]}"#;
    assert_eq!(
        check_capability(json, &COMMANDS),
        Ok(vec![Problem::MissingPermission(
            "staff_summary".to_string()
        )])
    );
}

#[test]
fn finds_a_permission_for_a_command_that_no_longer_exists() {
    let json = r#"{"identifier": "main", "windows": ["main"],
                   "permissions": ["allow-add-holding", "allow-staff-summary", "allow-mark-qualified"]}"#;
    assert_eq!(
        check_capability(json, &COMMANDS),
        Ok(vec![Problem::UnknownCommandPermission(
            "allow-mark-qualified".to_string()
        )])
    );
}

#[test]
fn plugin_permissions_are_not_mistaken_for_commands() {
    let json = r#"{"identifier": "main", "windows": ["main"],
                   "permissions": ["allow-add-holding", "allow-staff-summary", "fs:allow-read-text-file"]}"#;
    assert_eq!(check_capability(json, &COMMANDS), Ok(vec![]));
}

#[test]
fn finds_every_problem_sorted() {
    let json = r#"{"identifier": "default", "windows": ["*"],
                   "permissions": ["shell:allow-execute", "allow-old-command", "allow-add-holding", "shell:allow-execute"]}"#;
    assert_eq!(
        check_capability(json, &COMMANDS),
        Ok(vec![
            Problem::AllWindows,
            Problem::MissingPermission("staff_summary".to_string()),
            Problem::UnknownCommandPermission("allow-old-command".to_string()),
            Problem::Dangerous("shell:allow-execute".to_string()),
        ])
    );
}

#[test]
fn a_capability_for_no_windows() {
    let json = r#"{"identifier": "main", "windows": [], "permissions": ["allow-add-holding", "allow-staff-summary"]}"#;
    assert_eq!(
        check_capability(json, &COMMANDS),
        Ok(vec![Problem::NoWindows])
    );
}

#[test]
fn rejects_a_file_that_is_not_a_capability() {
    assert!(matches!(
        check_capability(r#"{"identifier": "main"}"#, &COMMANDS),
        Err(CommandError::Invalid { field, .. }) if field == "capability"
    ));
}
