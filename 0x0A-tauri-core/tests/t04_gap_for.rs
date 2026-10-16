//! Amina's transfer, asked from the app.

use chrono::NaiveDate;
use tauri_core::t00_command_error::CommandError;
use tauri_core::t01_inputs::NewHolding;
use tauri_core::t02_app_state::{AppState, add_holding};
use tauri_core::t04_gap_for::gap_for;
use twa_engine::model::RequirementState;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

/// The app's state with Amina's records entered through the add_holding command.
fn amina_in_the_app() -> AppState {
    let state = AppState::with_staff(&[(48213, "Amina Wanjiru"), (30303, "Baraka Otieno")]);
    for (code, completed_on) in [
        ("AIRSIDE", d(2025, 3, 10)),
        ("FIRE", d(2026, 3, 1)),
        ("DG-ICAO", d(2024, 11, 1)),
    ] {
        add_holding(
            &state,
            NewHolding {
                staff_number: 48213,
                code: code.to_string(),
                completed_on,
            },
        )
        .expect("Amina is in the app");
    }
    state
}

#[test]
fn amina_cannot_start_as_a_cargo_handler_yet() {
    let report = gap_for(
        &amina_in_the_app(),
        48213,
        "Cargo",
        "Handler",
        d(2026, 10, 19),
    )
    .unwrap();
    assert!(!report.is_ready());

    let lines: Vec<(&str, &RequirementState, Option<&str>)> = report
        .lines
        .iter()
        .map(|l| (l.code.as_str(), &l.state, l.satisfied_by.as_deref()))
        .collect();
    assert_eq!(
        lines,
        [
            (
                "AIRSIDE",
                &RequirementState::Met {
                    valid_until: d(2027, 3, 10)
                },
                Some("AIRSIDE")
            ),
            (
                "DG",
                &RequirementState::ExpiringSoon {
                    valid_until: d(2026, 11, 1),
                    days_left: 13
                },
                Some("DG-ICAO")
            ),
            ("FORKLIFT", &RequirementState::Missing, None),
        ]
    );
}

#[test]
fn amina_is_ready_for_her_current_role() {
    let report = gap_for(&amina_in_the_app(), 48213, "Ramp", "Agent", d(2026, 10, 19)).unwrap();
    assert!(report.is_ready());
}

#[test]
fn someone_with_nothing_is_missing_everything() {
    let report = gap_for(&amina_in_the_app(), 30303, "Ramp", "Agent", d(2026, 10, 19)).unwrap();
    assert!(
        report
            .lines
            .iter()
            .all(|l| l.state == RequirementState::Missing)
    );
}

#[test]
fn an_unknown_person_is_not_found() {
    assert_eq!(
        gap_for(&amina_in_the_app(), 99999, "Ramp", "Agent", d(2026, 10, 19)),
        Err(CommandError::NotFound("staff member 99999".to_string()))
    );
}

#[test]
fn an_unknown_role_is_invalid_input_for_the_screen() {
    let error = gap_for(
        &amina_in_the_app(),
        48213,
        "Cargo",
        "Pilot",
        d(2026, 10, 19),
    )
    .unwrap_err();
    assert_eq!(error.kind(), "invalid");
    assert_eq!(
        error,
        CommandError::Invalid {
            field: "role".to_string(),
            reason: "no requirements defined for Pilot in Cargo".to_string()
        }
    );
}
