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

use std::path::PathBuf;

use crate::model::{Command, UsageError};

pub fn parse_args(args: &[&str]) -> Result<Command, UsageError> {
    let mut args = args.iter().copied();
    let command = args.next().ok_or(UsageError::NoCommand)?;
    if command != "check" && command != "count" {
        return Err(UsageError::UnknownCommand(command.to_string()));
    }
    let file = PathBuf::from(args.next().ok_or_else(|| UsageError::MissingFile {
        command: command.to_string(),
    })?);

    let parsed = if command == "check" {
        Command::Check { file }
    } else {
        let mut station = None;
        if let Some(flag) = args.next() {
            if flag != "--station" {
                return Err(UsageError::Unexpected(flag.to_string()));
            }
            station = Some(args.next().ok_or(UsageError::MissingStation)?.to_string());
        }
        Command::Count { file, station }
    };

    match args.next() {
        Some(extra) => Err(UsageError::Unexpected(extra.to_string())),
        None => Ok(parsed),
    }
}
