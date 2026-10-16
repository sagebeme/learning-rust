//! Task 1: read a crate's dependencies from its `Cargo.toml`.
//!
//! A deliberately small reader, enough for the manifests in a workspace like this one:
//! - `package_name`: the `name = "..."` line in the `[package]` section, or `None`
//! - `internal_dependencies`: the dependency names in the `[dependencies]` section (not
//!   `[dev-dependencies]` or any other section) that are in `internal`, sorted, each once
//!
//! A dependency line looks like `name = "1"`, `name = { path = "../x" }` or
//! `name.workspace = true`; the name is everything before the first `=` or `.`, trimmed. Skip
//! blank lines and `#` comments. (Real tools use the `toml` crate or `cargo metadata`.)

use std::collections::BTreeSet;

/// Each meaningful line, paired with the `[section]` it's in.
fn lines_by_section(manifest: &str) -> Vec<(&str, &str)> {
    let mut section = "";
    let mut lines = Vec::new();
    for line in manifest.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = name.trim();
        } else {
            lines.push((section, line));
        }
    }
    lines
}

fn key_of(line: &str) -> &str {
    let end = line.find(['=', '.']).unwrap_or(line.len());
    line[..end].trim()
}

pub fn package_name(manifest: &str) -> Option<String> {
    lines_by_section(manifest)
        .into_iter()
        .find(|(section, line)| *section == "package" && key_of(line) == "name")
        .and_then(|(_, line)| line.split_once('='))
        .map(|(_, value)| value.trim().trim_matches('"').to_string())
}

pub fn internal_dependencies(manifest: &str, internal: &[&str]) -> Vec<String> {
    let names: BTreeSet<&str> = lines_by_section(manifest)
        .into_iter()
        .filter(|(section, _)| *section == "dependencies")
        .map(|(_, line)| key_of(line))
        .filter(|name| internal.contains(name))
        .collect();
    names.into_iter().map(str::to_string).collect()
}
