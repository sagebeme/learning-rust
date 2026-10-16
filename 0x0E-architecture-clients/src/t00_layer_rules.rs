//! Task 0: turn "who may depend on whom" into a check that runs on every pull request.
//!
//! `actual` maps each crate to the internal crates it depends on. `allowed` maps each crate to
//! the internal crates it is *allowed* to depend on. Report, sorted, each once:
//! - `UnknownCrate(name)` for a crate in `actual` that has no entry in `allowed` (every new crate
//!   needs a decision about where it sits)
//! - `NotAllowed { from, to }` for each dependency that isn't allowed
//!
//! For TWA, the rules look like: the engine depends on nothing internal; the database layer may use
//! the engine; the server may use both; nothing may depend on the server or the app.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Violation {
    UnknownCrate(String),
    NotAllowed { from: String, to: String },
}

pub fn check_layers(
    actual: &BTreeMap<String, Vec<String>>,
    allowed: &BTreeMap<String, Vec<String>>,
) -> Vec<Violation> {
    todo!()
}
