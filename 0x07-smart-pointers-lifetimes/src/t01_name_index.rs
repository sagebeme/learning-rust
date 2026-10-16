//! Task 1: an index that borrows names instead of copying them.
//!
//! `NameIndex::new(text)` takes text with one name per line, trimmed, skipping blank lines, and
//! keeps `&str` slices of it. Then:
//! - `starting_with(prefix)`: names starting with `prefix`, in the order they appear
//! - `longest()`: the longest name (in characters), the first if there's a tie
//!
//! The results borrow from the original *text*, not from the index, so they stay usable after the
//! index is gone. That's what the `'a` in the signatures says.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

pub struct NameIndex<'a> {
    names: Vec<&'a str>,
}

impl<'a> NameIndex<'a> {
    pub fn new(text: &'a str) -> NameIndex<'a> {
        todo!()
    }

    pub fn starting_with(&self, prefix: &str) -> Vec<&'a str> {
        todo!()
    }

    pub fn longest(&self) -> Option<&'a str> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
