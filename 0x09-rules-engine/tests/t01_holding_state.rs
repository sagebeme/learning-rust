mod common;

use common::{catalogue, d, holding, today, used};
use rules_engine::model::RequirementState;
use rules_engine::t01_holding_state::holding_state;

fn state_of(code: &str, h: &rules_engine::model::Holding) -> RequirementState {
    let catalogue = catalogue();
    holding_state(catalogue.get(code).expect("in the catalogue"), h, today())
}

#[test]
fn met_well_before_expiry() {
    // AIRSIDE: 24 months, warn 30 days. Valid until 10 Mar 2027: 145 days left on 16 Oct 2026.
    assert_eq!(
        state_of("AIRSIDE", &holding("AIRSIDE", d(2025, 3, 10))),
        RequirementState::Met {
            valid_until: d(2027, 3, 10)
        }
    );
}

#[test]
fn expiring_soon_inside_the_warning_window() {
    // DG-ICAO: warn 60 days. Valid until 1 Nov 2026: 16 days left.
    assert_eq!(
        state_of("DG-ICAO", &holding("DG-ICAO", d(2024, 11, 1))),
        RequirementState::ExpiringSoon {
            valid_until: d(2026, 11, 1),
            days_left: 16
        }
    );
}

#[test]
fn the_last_valid_day_is_expiring_soon_not_expired() {
    assert_eq!(
        state_of("AIRSIDE", &holding("AIRSIDE", d(2024, 10, 16))),
        RequirementState::ExpiringSoon {
            valid_until: d(2026, 10, 16),
            days_left: 0
        }
    );
}

#[test]
fn in_grace_up_to_and_including_the_last_grace_day() {
    // DG: 30 days' grace. Valid until 20 Sep 2026: 26 days overdue.
    assert_eq!(
        state_of("DG", &holding("DG", d(2024, 9, 20))),
        RequirementState::InGrace {
            valid_until: d(2026, 9, 20),
            days_overdue: 26
        }
    );
    // Exactly 30 days overdue: valid until 16 Sep 2026.
    assert!(matches!(
        state_of("DG", &holding("DG", d(2024, 9, 16))),
        RequirementState::InGrace {
            days_overdue: 30,
            ..
        }
    ));
}

#[test]
fn expired_after_the_grace_period() {
    assert_eq!(
        state_of("DG", &holding("DG", d(2024, 9, 15))),
        RequirementState::Expired {
            valid_until: d(2026, 9, 15)
        }
    );
    // FIRE has no grace at all: one day late is expired.
    assert_eq!(
        state_of("FIRE", &holding("FIRE", d(2025, 10, 15))),
        RequirementState::Expired {
            valid_until: d(2026, 10, 15)
        }
    );
}

#[test]
fn lapsed_when_not_used_recently_enough() {
    // FORKLIFT: recency 90 days. Last used 1 Jun 2026: 137 days ago.
    assert_eq!(
        state_of("FORKLIFT", &used("FORKLIFT", d(2026, 1, 5), d(2026, 6, 1))),
        RequirementState::Lapsed {
            last_used_on: d(2026, 6, 1)
        }
    );
}

#[test]
fn recency_counts_from_completion_when_use_was_never_recorded() {
    // Completed 18 Jul 2026, never recorded as used: exactly 90 days, still current.
    assert!(matches!(
        state_of("FORKLIFT", &holding("FORKLIFT", d(2026, 7, 18))),
        RequirementState::Met { .. }
    ));
    // Completed 17 Jul 2026: 91 days, lapsed.
    assert_eq!(
        state_of("FORKLIFT", &holding("FORKLIFT", d(2026, 7, 17))),
        RequirementState::Lapsed {
            last_used_on: d(2026, 7, 17)
        }
    );
}

#[test]
fn expiry_is_checked_before_recency() {
    // Expired 6 days ago (inside 14 days' grace), and not used for ages: grace wins.
    assert_eq!(
        state_of(
            "FORKLIFT",
            &used("FORKLIFT", d(2025, 10, 10), d(2025, 11, 1))
        ),
        RequirementState::InGrace {
            valid_until: d(2026, 10, 10),
            days_overdue: 6
        }
    );
}
