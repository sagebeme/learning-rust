//! An architecture test aimed at a real workspace: this one. It's the same check a TWA workspace
//! would run in CI, with this repo's rules:
//! - the engine (`twa-engine`, or your own `rules-engine`) depends on nothing internal
//! - weeks 5–7 (`tauri-core`, `databases`, `async-backend`, `jobs-alerts`) may use the engine,
//!   and nothing else
//! - every other project stands alone
//! - a crate not listed here at all is a violation: adding one needs a decision first

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use architecture_clients::t00_layer_rules::{Violation, check_layers};
use architecture_clients::t01_read_manifest::{internal_dependencies, package_name};

/// Every crate in this workspace, and the internal crates it may depend on.
fn rules() -> BTreeMap<String, Vec<String>> {
    let engine = ["twa-engine", "rules-engine"];
    let standalone = [
        "hello-cargo",
        "values-control",
        "functions-modules",
        "ownership-memory",
        "structs-enums-match",
        "errors-files-cli",
        "collections-iterators",
        "smart-pointers-lifetimes",
        "traits-dates-testing",
        "rules-engine",
        "twa-engine",
        "architecture-clients",
        "xtask",
    ];
    let uses_engine = ["tauri-core", "databases", "async-backend", "jobs-alerts"];

    let mut rules: BTreeMap<String, Vec<String>> = standalone
        .iter()
        .map(|name| (name.to_string(), Vec::new()))
        .collect();
    for name in uses_engine {
        rules.insert(
            name.to_string(),
            engine.iter().map(|e| e.to_string()).collect(),
        );
    }
    rules
}

fn workspace_manifests() -> BTreeMap<String, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the project sits inside the workspace");
    let mut manifests = BTreeMap::new();
    for entry in fs::read_dir(root).expect("the workspace folder is readable") {
        let folder = entry.expect("a folder entry").path();
        let path = folder.join("Cargo.toml");
        if folder.is_dir() && path.exists() {
            let text = fs::read_to_string(&path).expect("a readable manifest");
            let name = package_name(&text).expect("every project has a name");
            manifests.insert(name, text);
        }
    }
    manifests
}

#[test]
fn every_crate_follows_the_layer_rules() {
    let manifests = workspace_manifests();
    assert!(
        manifests.len() >= 17,
        "found only {} crates",
        manifests.len()
    );

    let internal: Vec<&str> = manifests.keys().map(String::as_str).collect();
    let actual: BTreeMap<String, Vec<String>> = manifests
        .iter()
        .map(|(name, text)| (name.clone(), internal_dependencies(text, &internal)))
        .collect();

    assert_eq!(check_layers(&actual, &rules()), vec![]);
}

#[test]
fn weeks_5_to_7_really_do_use_the_engine() {
    let manifests = workspace_manifests();
    let internal: Vec<&str> = manifests.keys().map(String::as_str).collect();
    for name in ["tauri-core", "databases", "async-backend", "jobs-alerts"] {
        let deps = internal_dependencies(&manifests[name], &internal);
        assert_eq!(deps.len(), 1, "{name} should use exactly one engine");
    }
}

#[test]
fn the_rules_would_catch_the_engine_reaching_into_the_server() {
    let actual = BTreeMap::from([("twa-engine".to_string(), vec!["async-backend".to_string()])]);
    assert_eq!(
        check_layers(&actual, &rules()),
        vec![Violation::NotAllowed {
            from: "twa-engine".to_string(),
            to: "async-backend".to_string()
        }]
    );
}
