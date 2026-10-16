//! Given: the types every task uses. Read them, but you don't need to change them.

use std::io;
use std::path::PathBuf;

/// One staff member, as read from a roster file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaffRecord {
    pub staff_number: u32,
    pub name: String,
    pub station: String,
}

/// The stations a roster may mention.
pub const STATIONS: [&str; 4] = ["NBO", "MBA", "KIS", "EDL"];

/// Everything that can be wrong with one line of a roster. Each variant carries the line number,
/// so the person fixing the file knows where to look.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("line {line}: expected 3 fields, found {found}")]
    WrongFieldCount { line: usize, found: usize },
    #[error("line {line}: staff number {value:?} is not a whole number")]
    BadStaffNumber { line: usize, value: String },
    #[error("line {line}: the name is empty")]
    EmptyName { line: usize },
    #[error("line {line}: unknown station {code:?}")]
    UnknownStation { line: usize, code: String },
}

/// Everything that can go wrong loading a roster file: reading it, or what's inside it.
#[derive(Debug, thiserror::Error)]
pub enum RosterError {
    #[error("could not read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    Parse(#[from] ParseError),
}

/// The commands the `roster` tool understands (task 100).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// `roster check <file>`
    Check { file: PathBuf },
    /// `roster count <file> [--station <CODE>]`
    Count {
        file: PathBuf,
        station: Option<String>,
    },
}

/// What's wrong with how the tool was called (task 100).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum UsageError {
    #[error("no command given. Usage: roster <check|count> <file> [--station <CODE>]")]
    NoCommand,
    #[error("unknown command {0:?}. Use check or count")]
    UnknownCommand(String),
    #[error("{command} needs a file")]
    MissingFile { command: String },
    #[error("--station needs a station code")]
    MissingStation,
    #[error("unexpected argument {0:?}")]
    Unexpected(String),
}
