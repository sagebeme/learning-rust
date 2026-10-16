use architecture_clients::t01_read_manifest::{internal_dependencies, package_name};

const MANIFEST: &str = r#"
[package]
name = "twa-server"
version = "0.1.0"
edition.workspace = true

[dependencies]
# the engine and the database layer
twa-engine.workspace = true
twa-db = { path = "../twa-db" }
axum = "0.8"
twa-engine = { path = "../twa-engine" }

[dev-dependencies]
twa-test-support = { path = "../twa-test-support" }

[lints]
workspace = true
"#;

const INTERNAL: [&str; 4] = ["twa-engine", "twa-db", "twa-server", "twa-test-support"];

#[test]
fn reads_the_package_name() {
    assert_eq!(package_name(MANIFEST), Some("twa-server".to_string()));
}

#[test]
fn no_package_section_means_no_name() {
    assert_eq!(package_name("[workspace]\nmembers = [\"a\"]\n"), None);
}

#[test]
fn reads_internal_dependencies_sorted_and_once() {
    assert_eq!(
        internal_dependencies(MANIFEST, &INTERNAL),
        ["twa-db", "twa-engine"]
    );
}

#[test]
fn ignores_dev_dependencies_and_outside_crates() {
    let deps = internal_dependencies(MANIFEST, &INTERNAL);
    assert!(!deps.contains(&"twa-test-support".to_string()));
    assert!(!deps.contains(&"axum".to_string()));
}

#[test]
fn a_manifest_without_dependencies() {
    assert!(internal_dependencies("[package]\nname = \"x\"\n", &INTERNAL).is_empty());
}
