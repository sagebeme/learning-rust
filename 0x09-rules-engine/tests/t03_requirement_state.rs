mod common;

use common::{catalogue, d, holding, today};
use rules_engine::model::RequirementState;
use rules_engine::t03_requirement_state::requirement_state;

#[test]
fn usable_with_its_prerequisite() {
    let holdings = [
        holding("FORKLIFT", d(2026, 9, 1)),
        holding("AIRSIDE", d(2026, 1, 1)),
    ];
    let (state, by) = requirement_state(&catalogue(), "FORKLIFT", &holdings, today()).unwrap();
    assert_eq!(
        state,
        RequirementState::Met {
            valid_until: d(2027, 9, 1)
        }
    );
    assert_eq!(by.as_deref(), Some("FORKLIFT"));
}

#[test]
fn a_missing_prerequisite_blocks_a_usable_holding() {
    // DG is in grace (usable), but AIRSIDE isn't held.
    let holdings = [holding("DG", d(2024, 9, 20))];
    assert_eq!(
        requirement_state(&catalogue(), "DG", &holdings, today()),
        Ok((
            RequirementState::PrerequisiteMissing {
                missing: vec!["AIRSIDE".to_string()]
            },
            Some("DG".to_string())
        ))
    );
}

#[test]
fn an_expired_prerequisite_blocks_too() {
    let holdings = [
        holding("DG", d(2026, 1, 1)),
        holding("AIRSIDE", d(2024, 1, 1)), // expired January 2026
    ];
    let (state, _) = requirement_state(&catalogue(), "DG", &holdings, today()).unwrap();
    assert_eq!(
        state,
        RequirementState::PrerequisiteMissing {
            missing: vec!["AIRSIDE".to_string()]
        }
    );
}

#[test]
fn an_unusable_state_is_reported_as_it_is() {
    // Expired DG and no AIRSIDE: the answer is "expired", not "prerequisite missing".
    let holdings = [holding("DG", d(2023, 1, 1))];
    let (state, _) = requirement_state(&catalogue(), "DG", &holdings, today()).unwrap();
    assert!(matches!(state, RequirementState::Expired { .. }));
}

#[test]
fn an_equivalent_still_needs_the_required_types_prerequisites() {
    // DG-ICAO has no prerequisites of its own, but it's standing in for DG, which needs AIRSIDE.
    let holdings = [holding("DG-ICAO", d(2026, 2, 1))];
    let (state, by) = requirement_state(&catalogue(), "DG", &holdings, today()).unwrap();
    assert_eq!(
        state,
        RequirementState::PrerequisiteMissing {
            missing: vec!["AIRSIDE".to_string()]
        }
    );
    assert_eq!(by.as_deref(), Some("DG-ICAO"));
}
