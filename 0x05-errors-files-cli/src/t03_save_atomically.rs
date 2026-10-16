//! Task 3: save a file so a power cut can never leave it half-written.
//!
//! 1. Write the contents to a temporary file **in the same folder**, named `.<file name>.tmp`.
//! 2. Flush it to the disk with `sync_all`.
//! 3. Rename it over the real file. A rename within one folder is all-or-nothing: readers see
//!    either the old file or the new one, never half of each.
//!
//! If anything fails, return the `io::Error`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn save_atomically(path: &Path, contents: &str) -> io::Result<()> {
    todo!()
}
