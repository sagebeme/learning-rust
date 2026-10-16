use chrono::NaiveDate;
use tauri_core::t00_command_error::CommandError;
use tauri_core::t01_inputs::{NewHolding, parse_new_holding};

fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 10, 16).expect("a real date")
}

fn field_of(result: Result<NewHolding, CommandError>) -> String {
    match result {
        Err(CommandError::Invalid { field, .. }) => field,
        other => panic!("expected an Invalid error, got {other:?}"),
    }
}

#[test]
fn reads_camel_case_json_from_the_screen() {
    let json = r#"{"staffNumber": 48213, "code": " DG ", "completedOn": "2026-03-01"}"#;
    assert_eq!(
        parse_new_holding(json, today()),
        Ok(NewHolding {
            staff_number: 48213,
            code: "DG".to_string(),
            completed_on: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
        })
    );
}

#[test]
fn snake_case_keys_are_not_accepted() {
    let json = r#"{"staff_number": 48213, "code": "DG", "completed_on": "2026-03-01"}"#;
    assert_eq!(field_of(parse_new_holding(json, today())), "body");
}

#[test]
fn a_misspelled_key_is_an_error_not_a_silent_gap() {
    let json = r#"{"staffNumber": 1, "code": "DG", "completedOn": "2026-03-01", "staffNumbr": 2}"#;
    assert_eq!(field_of(parse_new_holding(json, today())), "body");
}

#[test]
fn wrong_types_and_bad_dates_are_body_errors() {
    for json in [
        r#"{"staffNumber": "48213", "code": "DG", "completedOn": "2026-03-01"}"#,
        r#"{"staffNumber": -1, "code": "DG", "completedOn": "2026-03-01"}"#,
        r#"{"staffNumber": 1, "code": "DG", "completedOn": "01/03/2026"}"#,
        r#"{"staffNumber": 1, "code": "DG"}"#,
        "not json",
    ] {
        assert_eq!(field_of(parse_new_holding(json, today())), "body", "{json}");
    }
}

#[test]
fn rejects_bad_codes() {
    for code in ["", "D", "dg", "DG CERT", "ABCDEFGHIJKLM", "DG!"] {
        let json =
            format!(r#"{{"staffNumber": 1, "code": "{code}", "completedOn": "2026-03-01"}}"#);
        assert_eq!(
            field_of(parse_new_holding(&json, today())),
            "code",
            "{code:?}"
        );
    }
}

#[test]
fn accepts_digits_and_dashes_in_codes() {
    let json = r#"{"staffNumber": 1, "code": "DG-ICAO-2", "completedOn": "2026-03-01"}"#;
    assert!(parse_new_holding(json, today()).is_ok());
}

#[test]
fn rejects_a_completion_date_in_the_future_but_allows_today() {
    let tomorrow = r#"{"staffNumber": 1, "code": "DG", "completedOn": "2026-10-17"}"#;
    assert_eq!(
        field_of(parse_new_holding(tomorrow, today())),
        "completedOn"
    );
    let on_the_day = r#"{"staffNumber": 1, "code": "DG", "completedOn": "2026-10-16"}"#;
    assert!(parse_new_holding(on_the_day, today()).is_ok());
}
