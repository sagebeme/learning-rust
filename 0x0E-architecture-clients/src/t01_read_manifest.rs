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

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::BTreeSet;

pub fn package_name(manifest: &str) -> Option<String> {
    todo!()
}

pub fn internal_dependencies(manifest: &str, internal: &[&str]) -> Vec<String> {
    todo!()
}
