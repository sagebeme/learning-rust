//! Task 100 (advanced): refuse a broken catalogue before the engine ever sees it.
//!
//! Check, in this order, and return the first problem:
//! 1. **Duplicate codes:** the first code that appears twice, in input order → `DuplicateCode`.
//! 2. **Unknown references:** going through types in code order, and each type's
//!    `prerequisites` then `equivalents` in listed order, the first code that isn't in the
//!    catalogue → `UnknownReference { code, refers_to }`.
//! 3. **Prerequisite cycles:** if the prerequisites can't be put in a training order (as in
//!    0x07 task 100) → `PrerequisiteCycle` with every code that can't be ordered, sorted.
//!
//! Otherwise return the catalogue.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::model::{Catalogue, CatalogueError, QualificationType};

pub fn validate_catalogue(types: Vec<QualificationType>) -> Result<Catalogue, CatalogueError> {
    let mut seen = HashSet::new();
    for qualification in &types {
        if !seen.insert(qualification.code.as_str()) {
            return Err(CatalogueError::DuplicateCode(qualification.code.clone()));
        }
    }

    let catalogue = Catalogue::from_types(types);

    for qualification in catalogue.types.values() {
        let references = qualification
            .prerequisites
            .iter()
            .chain(&qualification.equivalents);
        for reference in references {
            if catalogue.get(reference).is_none() {
                return Err(CatalogueError::UnknownReference {
                    code: qualification.code.clone(),
                    refers_to: reference.clone(),
                });
            }
        }
    }

    // Kahn's algorithm over the prerequisites: repeatedly remove codes with nothing left to wait
    // for. Whatever can't be removed is in a cycle, or waits on one.
    let mut waiting: BTreeMap<&str, BTreeSet<&str>> = catalogue
        .types
        .values()
        .map(|q| {
            (
                q.code.as_str(),
                q.prerequisites.iter().map(String::as_str).collect(),
            )
        })
        .collect();
    loop {
        let ready: Vec<&str> = waiting
            .iter()
            .filter(|(_, needs)| needs.is_empty())
            .map(|(code, _)| *code)
            .collect();
        if ready.is_empty() {
            break;
        }
        for code in ready {
            waiting.remove(code);
            for needs in waiting.values_mut() {
                needs.remove(code);
            }
        }
    }
    if !waiting.is_empty() {
        let stuck = waiting.keys().map(|code| code.to_string()).collect();
        return Err(CatalogueError::PrerequisiteCycle(stuck));
    }

    Ok(catalogue)
}
