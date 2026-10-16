//! Task 100 (advanced): read the command line for a small `roster` tool.
//!
//! The arguments don't include the program name:
//! - `["check", "staff.csv"]` → `Command::Check`
//! - `["count", "staff.csv"]` → `Command::Count { station: None }`
//! - `["count", "staff.csv", "--station", "NBO"]` → `Command::Count { station: Some("NBO") }`
//!
//! Errors: nothing at all → `NoCommand`; an unknown first word → `UnknownCommand`; no file →
//! `MissingFile { command }`; `--station` with nothing after it → `MissingStation`; anything left
//! over (including `--station` on `check`) → `Unexpected` with the first leftover argument.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::path::PathBuf;

use crate::model::{Command, UsageError};

pub fn parse_args(args: &[&str]) -> Result<Command, UsageError> {
    todo!()
}
