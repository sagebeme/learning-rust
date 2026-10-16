use std::collections::BTreeMap;

use architecture_clients::t00_layer_rules::{Violation, check_layers};

fn map(entries: &[(&str, &[&str])]) -> BTreeMap<String, Vec<String>> {
    entries
        .iter()
        .map(|(name, deps)| {
            (
                name.to_string(),
                deps.iter().map(|d| d.to_string()).collect(),
            )
        })
        .collect()
}

/// TWA's intended layering.
fn twa_rules() -> BTreeMap<String, Vec<String>> {
    map(&[
        ("twa-engine", &[]),
        ("twa-db", &["twa-engine"]),
        ("twa-server", &["twa-engine", "twa-db"]),
        ("twa-app-core", &["twa-engine"]),
    ])
}

#[test]
fn a_clean_workspace_has_no_violations() {
    let actual = map(&[
        ("twa-engine", &[]),
        ("twa-db", &["twa-engine"]),
        ("twa-server", &["twa-db", "twa-engine"]),
        ("twa-app-core", &["twa-engine"]),
    ]);
    assert_eq!(check_layers(&actual, &twa_rules()), vec![]);
}

#[test]
fn the_engine_reaching_for_the_database_is_caught() {
    let actual = map(&[("twa-engine", &["twa-db"]), ("twa-db", &["twa-engine"])]);
    assert_eq!(
        check_layers(&actual, &twa_rules()),
        vec![Violation::NotAllowed {
            from: "twa-engine".to_string(),
            to: "twa-db".to_string()
        }]
    );
}

#[test]
fn the_app_must_not_depend_on_the_server() {
    let actual = map(&[("twa-app-core", &["twa-engine", "twa-server"])]);
    assert_eq!(
        check_layers(&actual, &twa_rules()),
        vec![Violation::NotAllowed {
            from: "twa-app-core".to_string(),
            to: "twa-server".to_string()
        }]
    );
}

#[test]
fn a_new_crate_needs_a_decision_first() {
    let actual = map(&[("twa-reports", &["twa-db"]), ("twa-engine", &[])]);
    assert_eq!(
        check_layers(&actual, &twa_rules()),
        vec![Violation::UnknownCrate("twa-reports".to_string())]
    );
}

#[test]
fn every_problem_sorted_and_once() {
    let actual = map(&[
        ("twa-engine", &["twa-server", "twa-db", "twa-db"]),
        ("zz-tool", &[]),
    ]);
    assert_eq!(
        check_layers(&actual, &twa_rules()),
        vec![
            Violation::UnknownCrate("zz-tool".to_string()),
            Violation::NotAllowed {
                from: "twa-engine".to_string(),
                to: "twa-db".to_string()
            },
            Violation::NotAllowed {
                from: "twa-engine".to_string(),
                to: "twa-server".to_string()
            },
        ]
    );
}
