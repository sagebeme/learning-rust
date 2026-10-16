//! An architecture test aimed at a real workspace: this one. Every course project must stand on
//! its own, so no project may depend on another. This is the same check a TWA workspace would run
//! in CI with its own rules.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use architecture_clients::t00_layer_rules::check_layers;
use architecture_clients::t01_read_manifest::{internal_dependencies, package_name};

#[test]
fn no_course_project_depends_on_another() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the project sits inside the workspace");

    let mut manifests = BTreeMap::new();
    for entry in fs::read_dir(root).expect("the workspace folder is readable") {
        let path = entry.expect("a folder entry").path().join("Cargo.toml");
        let is_project = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("0x") || n == "xtask");
        if is_project && path.exists() {
            let text = fs::read_to_string(&path).expect("a readable manifest");
            let name = package_name(&text).expect("every project has a name");
            manifests.insert(name, text);
        }
    }
    assert!(
        manifests.len() >= 15,
        "found only {} projects",
        manifests.len()
    );

    let internal: Vec<&str> = manifests.keys().map(String::as_str).collect();
    let actual: BTreeMap<String, Vec<String>> = manifests
        .iter()
        .map(|(name, text)| (name.clone(), internal_dependencies(text, &internal)))
        .collect();
    // The rule for this repo: nobody may depend on anybody.
    let allowed: BTreeMap<String, Vec<String>> = manifests
        .keys()
        .map(|name| (name.clone(), Vec::new()))
        .collect();

    assert_eq!(check_layers(&actual, &allowed), vec![]);
}
