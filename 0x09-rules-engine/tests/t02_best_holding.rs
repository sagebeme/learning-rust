mod common;

use common::{catalogue, d, holding, today};
use rules_engine::model::{EngineError, RequirementState};
use rules_engine::t02_best_holding::best_holding;

#[test]
fn uses_the_qualification_itself() {
    let holdings = [holding("FIRE", d(2026, 3, 1))];
    assert_eq!(
        best_holding(&catalogue(), "FIRE", &holdings, today()),
        Ok((
            RequirementState::Met {
                valid_until: d(2027, 3, 1)
            },
            Some("FIRE".to_string())
        ))
    );
}

#[test]
fn an_equivalent_counts_and_is_judged_by_its_own_type() {
    // DG-ICAO has warn 60 and no grace: 16 days left is "expiring soon".
    let holdings = [holding("DG-ICAO", d(2024, 11, 1))];
    assert_eq!(
        best_holding(&catalogue(), "DG", &holdings, today()),
        Ok((
            RequirementState::ExpiringSoon {
                valid_until: d(2026, 11, 1),
                days_left: 16
            },
            Some("DG-ICAO".to_string())
        ))
    );
}

#[test]
fn equivalence_only_goes_one_way() {
    // DG lists DG-ICAO as an equivalent, not the other way round.
    let holdings = [holding("DG", d(2026, 1, 1))];
    assert_eq!(
        best_holding(&catalogue(), "DG-ICAO", &holdings, today()),
        Ok((RequirementState::Missing, None))
    );
}

#[test]
fn picks_the_better_of_two_holdings() {
    let holdings = [
        holding("DG", d(2023, 1, 10)),     // expired
        holding("DG-ICAO", d(2026, 2, 1)), // met
    ];
    let (state, by) = best_holding(&catalogue(), "DG", &holdings, today()).unwrap();
    assert_eq!(
        state,
        RequirementState::Met {
            valid_until: d(2028, 2, 1)
        }
    );
    assert_eq!(by.as_deref(), Some("DG-ICAO"));
}

#[test]
fn breaks_a_tie_by_the_later_expiry_then_by_code() {
    let later = [
        holding("DG", d(2026, 1, 1)),
        holding("DG-ICAO", d(2026, 2, 1)),
    ];
    let (_, by) = best_holding(&catalogue(), "DG", &later, today()).unwrap();
    assert_eq!(by.as_deref(), Some("DG-ICAO"));

    let same_day = [
        holding("DG-ICAO", d(2026, 2, 1)),
        holding("DG", d(2026, 2, 1)),
    ];
    let (_, by) = best_holding(&catalogue(), "DG", &same_day, today()).unwrap();
    assert_eq!(by.as_deref(), Some("DG"));
}

#[test]
fn ignores_holdings_that_do_not_apply() {
    let holdings = [holding("FIRE", d(2026, 3, 1))];
    assert_eq!(
        best_holding(&catalogue(), "AIRSIDE", &holdings, today()),
        Ok((RequirementState::Missing, None))
    );
}

#[test]
fn rejects_an_unknown_requirement() {
    assert_eq!(
        best_holding(&catalogue(), "PILOT", &[], today()),
        Err(EngineError::UnknownQualification("PILOT".to_string()))
    );
}
