//! Task 2: one log, many owners, with `Rc<RefCell<...>>`.
//!
//! Several parts of the app need to write alerts into the *same* log. `AlertLog` is a cheap handle:
//! `handle()` gives another handle to the same log, and anything recorded through any handle is
//! seen by all of them.
//! - `record(&self, message)` adds a message (note: `&self`, not `&mut self`)
//! - `entries(&self)` returns a copy of all messages, oldest first
//! - `handles(&self)` returns how many handles to this log exist right now

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

pub struct AlertLog {
    entries: Rc<RefCell<Vec<String>>>,
}

impl AlertLog {
    pub fn new() -> AlertLog {
        todo!()
    }

    pub fn handle(&self) -> AlertLog {
        todo!()
    }

    pub fn record(&self, message: &str) {
        todo!()
    }

    pub fn entries(&self) -> Vec<String> {
        todo!()
    }

    pub fn handles(&self) -> usize {
        todo!()
    }
}

impl Default for AlertLog {
    fn default() -> Self {
        AlertLog::new()
    }
}
