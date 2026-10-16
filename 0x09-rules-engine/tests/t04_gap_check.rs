mod common;

use common::{catalogue, d, holding, matrix, today, used};
use proptest::prelude::*;
use rules_engine::model::{EngineError, Holding, RequirementState};
use rules_engine::t04_gap_check::gap_check;

#[test]
fn one_line_per_requirement_sorted_by_code() {
    let report = gap_check(&catalogue(), &matrix(), "Cargo", "Handler", &[], today()).unwrap();
    let codes: Vec<&str> = report.lines.iter().map(|l| l.code.as_str()).collect();
    assert_eq!(codes, ["AIRSIDE", "DG", "FORKLIFT"]);
    assert!(
        report
            .lines
            .iter()
            .all(|l| l.state == RequirementState::Missing)
    );
    assert!(!report.is_ready());
    assert_eq!(report.department, "Cargo");
    assert_eq!(report.role, "Handler");
}

#[test]
fn a_ready_transfer() {
    let holdings = [
        holding("AIRSIDE", d(2026, 1, 1)),
        holding("DG-ICAO", d(2026, 2, 1)),
        holding("FORKLIFT", d(2026, 9, 1)),
    ];
    let report = gap_check(
        &catalogue(),
        &matrix(),
        "Cargo",
        "Handler",
        &holdings,
        today(),
    )
    .unwrap();
    assert!(report.is_ready(), "{report:#?}");
}

#[test]
fn a_lapsed_forklift_licence_blocks_the_transfer() {
    let holdings = [
        holding("AIRSIDE", d(2026, 1, 1)),
        holding("DG", d(2026, 1, 1)),
        used("FORKLIFT", d(2026, 1, 5), d(2026, 6, 1)),
    ];
    let report = gap_check(
        &catalogue(),
        &matrix(),
        "Cargo",
        "Handler",
        &holdings,
        today(),
    )
    .unwrap();
    assert!(!report.is_ready());
    assert!(matches!(
        report.lines[2].state,
        RequirementState::Lapsed { .. }
    ));
}

#[test]
fn rejects_a_role_with_no_requirements() {
    assert_eq!(
        gap_check(&catalogue(), &matrix(), "Cargo", "Pilot", &[], today()),
        Err(EngineError::UnknownRole {
            department: "Cargo".to_string(),
            role: "Pilot".to_string()
        })
    );
}

#[test]
fn a_requirement_listed_twice_appears_once() {
    let mut matrix = matrix();
    matrix.roles.entry("Ramp".into()).or_default().insert(
        "Lead".into(),
        vec!["FIRE".into(), "AIRSIDE".into(), "FIRE".into()],
    );
    let report = gap_check(&catalogue(), &matrix, "Ramp", "Lead", &[], today()).unwrap();
    assert_eq!(report.lines.len(), 2);
}

// The engine runs on every device and the server, and must always give the same answer. Here
// proptest generates random sets of holdings and checks that the order they arrive in never
// changes the report.
fn any_holding() -> impl Strategy<Value = Holding> {
    let codes = prop::sample::select(vec!["AIRSIDE", "DG", "DG-ICAO", "FORKLIFT", "FIRE"]);
    let date = (2022i32..2027, 1u32..=12, 1u32..=28).prop_map(|(y, m, day)| d(y, m, day));
    (codes, date.clone(), prop::option::of(date)).prop_map(|(code, completed_on, last_used_on)| {
        Holding {
            code: code.to_string(),
            completed_on,
            last_used_on,
        }
    })
}

proptest! {
    #[test]
    fn the_order_of_holdings_never_changes_the_report(
        holdings in prop::collection::vec(any_holding(), 0..8)
    ) {
        let forwards = gap_check(&catalogue(), &matrix(), "Cargo", "Handler", &holdings, today());
        let mut reversed = holdings.clone();
        reversed.reverse();
        let backwards = gap_check(&catalogue(), &matrix(), "Cargo", "Handler", &reversed, today());
        prop_assert_eq!(forwards, backwards);
    }
}
