//! Task 2: one log, many owners, with `Rc<RefCell<...>>`.
//!
//! Several parts of the app need to write alerts into the *same* log. `AlertLog` is a cheap handle:
//! `handle()` gives another handle to the same log, and anything recorded through any handle is
//! seen by all of them.
//! - `record(&self, message)` adds a message (note: `&self`, not `&mut self`)
//! - `entries(&self)` returns a copy of all messages, oldest first
//! - `handles(&self)` returns how many handles to this log exist right now

use std::cell::RefCell;
use std::rc::Rc;

pub struct AlertLog {
    entries: Rc<RefCell<Vec<String>>>,
}

impl AlertLog {
    pub fn new() -> AlertLog {
        AlertLog {
            entries: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn handle(&self) -> AlertLog {
        AlertLog {
            entries: Rc::clone(&self.entries),
        }
    }

    pub fn record(&self, message: &str) {
        self.entries.borrow_mut().push(message.to_string());
    }

    pub fn entries(&self) -> Vec<String> {
        self.entries.borrow().clone()
    }

    pub fn handles(&self) -> usize {
        Rc::strong_count(&self.entries)
    }
}

impl Default for AlertLog {
    fn default() -> Self {
        AlertLog::new()
    }
}
