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

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::model::{Catalogue, CatalogueError, QualificationType};

pub fn validate_catalogue(types: Vec<QualificationType>) -> Result<Catalogue, CatalogueError> {
    todo!()
}
