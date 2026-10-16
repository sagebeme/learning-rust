//! Task 100 (advanced): prerequisites as a graph, not a web of pointers.
//!
//! `require(qualification, prerequisite)` records that you need `prerequisite` before
//! `qualification`. Then:
//! - `has_cycle()`: true if some qualification ends up requiring itself, directly or through others
//! - `training_order()`: every qualification mentioned, ordered so each comes after all its
//!   prerequisites; among ones that could go next, alphabetical order. `None` if there's a cycle.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default)]
pub struct PrerequisiteGraph {
    /// qualification → the qualifications it needs first
    needs: BTreeMap<String, BTreeSet<String>>,
}

impl PrerequisiteGraph {
    pub fn new() -> PrerequisiteGraph {
        PrerequisiteGraph::default()
    }

    pub fn require(&mut self, qualification: &str, prerequisite: &str) {
        todo!()
    }

    pub fn has_cycle(&self) -> bool {
        todo!()
    }

    pub fn training_order(&self) -> Option<Vec<String>> {
        todo!()
    }
}
