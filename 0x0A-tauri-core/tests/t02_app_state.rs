use chrono::NaiveDate;
use serde_json::json;
use tauri_core::t00_command_error::CommandError;
use tauri_core::t01_inputs::NewHolding;
use tauri_core::t02_app_state::{AppState, StaffSummary, add_holding, staff_summary};

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

fn new_holding(staff_number: u32, code: &str, completed_on: NaiveDate) -> NewHolding {
    NewHolding {
        staff_number,
        code: code.to_string(),
        completed_on,
    }
}

fn state() -> AppState {
    AppState::with_staff(&[(48213, "Amina Wanjiru"), (10007, "Kip Ruto")])
}

#[test]
fn summarises_a_staff_member() {
    assert_eq!(
        staff_summary(&state(), 48213),
        Ok(StaffSummary {
            staff_number: 48213,
            name: "Amina Wanjiru".to_string(),
            codes: vec![],
        })
    );
}

#[test]
fn an_unknown_staff_member_is_not_found() {
    assert_eq!(
        staff_summary(&state(), 99999),
        Err(CommandError::NotFound("staff member 99999".to_string()))
    );
    assert_eq!(
        add_holding(&state(), new_holding(99999, "DG", d(2026, 1, 1))),
        Err(CommandError::NotFound("staff member 99999".to_string()))
    );
}

#[test]
fn adding_holdings_lists_codes_in_order() {
    let state = state();
    add_holding(&state, new_holding(48213, "FIRE", d(2026, 1, 1))).unwrap();
    let summary = add_holding(&state, new_holding(48213, "DG", d(2026, 2, 1))).unwrap();
    assert_eq!(summary.codes, ["DG", "FIRE"]);
    assert_eq!(staff_summary(&state, 48213).unwrap().codes, ["DG", "FIRE"]);
}

#[test]
fn an_older_record_never_replaces_a_newer_one() {
    let state = state();
    add_holding(&state, new_holding(48213, "DG", d(2026, 2, 1))).unwrap();
    // A late sync arrives with the previous certificate. The summary is unchanged.
    let summary = add_holding(&state, new_holding(48213, "DG", d(2024, 2, 1))).unwrap();
    assert_eq!(summary.codes, ["DG"]);
    let staff = state.lock_staff().unwrap();
    assert_eq!(staff[&48213].holdings["DG"], d(2026, 2, 1));
}

#[test]
fn a_newer_record_replaces_an_older_one() {
    let state = state();
    add_holding(&state, new_holding(48213, "DG", d(2024, 2, 1))).unwrap();
    add_holding(&state, new_holding(48213, "DG", d(2026, 2, 1))).unwrap();
    let staff = state.lock_staff().unwrap();
    assert_eq!(staff[&48213].holdings["DG"], d(2026, 2, 1));
}

#[test]
fn a_poisoned_lock_is_busy_not_a_crash() {
    let state = state();
    state.poison_for_test();
    assert_eq!(staff_summary(&state, 48213), Err(CommandError::Busy));
    assert_eq!(
        add_holding(&state, new_holding(48213, "DG", d(2026, 1, 1))),
        Err(CommandError::Busy)
    );
}

#[test]
fn the_summary_reaches_the_screen_in_camel_case() {
    let summary = staff_summary(&state(), 10007).unwrap();
    assert_eq!(
        serde_json::to_value(summary).unwrap(),
        json!({ "staffNumber": 10007, "name": "Kip Ruto", "codes": [] })
    );
}

#[test]
fn commands_on_many_threads_at_once_lose_nothing() {
    let state = state();
    std::thread::scope(|scope| {
        for i in 0..20 {
            let state = &state;
            scope.spawn(move || {
                add_holding(
                    state,
                    new_holding(48213, &format!("Q{i:02}"), d(2026, 1, 1)),
                )
                .unwrap();
            });
        }
    });
    assert_eq!(staff_summary(&state, 48213).unwrap().codes.len(), 20);
}
