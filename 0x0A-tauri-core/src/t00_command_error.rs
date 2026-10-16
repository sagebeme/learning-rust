//! Task 0: an error the screens can understand.
//!
//! When a Tauri command returns `Err`, Tauri serializes the error and sends it to the screen.
//! Serialize every `CommandError` as `{ "kind": ..., "message": ... }`:
//! - `kind()`: `"notFound"`, `"invalid"`, `"busy"` or `"internal"`, so the screen can choose what
//!   to show
//! - `message`: the error's `Display` text (written by the `#[error(...)]` lines)
//!
//! Implement `Serialize` by building an `ErrorPayload` and serializing that.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CommandError {
    #[error("{0} was not found")]
    NotFound(String),
    #[error("{field}: {reason}")]
    Invalid { field: String, reason: String },
    #[error("the app is busy, please try again")]
    Busy,
    #[error("something went wrong: {0}")]
    Internal(String),
}

/// What the screen receives.
#[derive(Serialize)]
struct ErrorPayload<'a> {
    kind: &'a str,
    message: String,
}

impl CommandError {
    pub fn kind(&self) -> &'static str {
        todo!()
    }
}

impl Serialize for CommandError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        todo!()
    }
}
