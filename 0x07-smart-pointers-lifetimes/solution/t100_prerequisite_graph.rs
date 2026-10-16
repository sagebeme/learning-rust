//! Task 100 (advanced): prerequisites as a graph, not a web of pointers.
//!
//! `require(qualification, prerequisite)` records that you need `prerequisite` before
//! `qualification`. Then:
//! - `has_cycle()`: true if some qualification ends up requiring itself, directly or through others
//! - `training_order()`: every qualification mentioned, ordered so each comes after all its
//!   prerequisites; among ones that could go next, alphabetical order. `None` if there's a cycle.

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
        self.needs
            .entry(qualification.to_string())
            .or_default()
            .insert(prerequisite.to_string());
        self.needs.entry(prerequisite.to_string()).or_default();
    }

    pub fn has_cycle(&self) -> bool {
        self.training_order().is_none()
    }

    pub fn training_order(&self) -> Option<Vec<String>> {
        // Kahn's algorithm: repeatedly take every qualification whose prerequisites are all done.
        // A BTreeMap/BTreeSet keeps "what could go next" in alphabetical order.
        let mut remaining: BTreeMap<&str, BTreeSet<&str>> = self
            .needs
            .iter()
            .map(|(qualification, prerequisites)| {
                (
                    qualification.as_str(),
                    prerequisites.iter().map(String::as_str).collect(),
                )
            })
            .collect();
        let mut order = Vec::with_capacity(remaining.len());

        while !remaining.is_empty() {
            let ready = remaining
                .iter()
                .find(|(_, prerequisites)| prerequisites.is_empty())
                .map(|(qualification, _)| *qualification)?;
            remaining.remove(ready);
            for prerequisites in remaining.values_mut() {
                prerequisites.remove(ready);
            }
            order.push(ready.to_string());
        }

        Some(order)
    }
}
