//! Task 3: save a file so a power cut can never leave it half-written.
//!
//! 1. Write the contents to a temporary file **in the same folder**, named `.<file name>.tmp`.
//! 2. Flush it to the disk with `sync_all`.
//! 3. Rename it over the real file. A rename within one folder is all-or-nothing: readers see
//!    either the old file or the new one, never half of each.
//!
//! If anything fails, return the `io::Error`.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn save_atomically(path: &Path, contents: &str) -> io::Result<()> {
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no file name"))?;
    let temp_path = path.with_file_name(format!(".{}.tmp", file_name.to_string_lossy()));

    let mut temp = File::create(&temp_path)?;
    temp.write_all(contents.as_bytes())?;
    temp.sync_all()?;
    drop(temp);

    fs::rename(&temp_path, path)
}
