//! Runs every case in `fixtures/transfers.json`. The same file can be run by any other build of
//! the engine (the app, the server) to prove they all give the same answers.

mod common;

use chrono::NaiveDate;
use rules_engine::model::{GapLine, Holding};
use rules_engine::t04_gap_check::gap_check;
use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    name: String,
    today: NaiveDate,
    department: String,
    role: String,
    holdings: Vec<Holding>,
    expected: Vec<GapLine>,
    ready: bool,
}

#[test]
fn every_shared_case_gives_the_expected_report() {
    let text = include_str!("fixtures/transfers.json");
    let cases: Vec<Case> = serde_json::from_str(text).expect("fixtures/transfers.json is valid");
    assert!(!cases.is_empty());

    for case in cases {
        let report = gap_check(
            &common::catalogue(),
            &common::matrix(),
            &case.department,
            &case.role,
            &case.holdings,
            case.today,
        )
        .unwrap_or_else(|error| panic!("{}: {error}", case.name));

        assert_eq!(report.lines, case.expected, "{}", case.name);
        assert_eq!(report.is_ready(), case.ready, "{}", case.name);
    }
}

#[test]
fn a_report_survives_a_round_trip_through_json() {
    let holdings = [common::holding("FIRE", common::d(2026, 3, 1))];
    let report = gap_check(
        &common::catalogue(),
        &common::matrix(),
        "Ramp",
        "Agent",
        &holdings,
        common::today(),
    )
    .unwrap();
    let json = serde_json::to_string(&report).unwrap();
    assert_eq!(
        serde_json::from_str::<rules_engine::model::GapReport>(&json).unwrap(),
        report
    );
}
