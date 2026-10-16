//! Repo chores that only CI needs. Run with `cargo xtask <command>`.
//!
//! `apply-solutions --ci` copies every project's `solution/*.rs` over the matching `src/*.rs`,
//! so CI can prove that every reference solution passes its tests. It overwrites your work, so it
//! refuses to run without `--ci`. Never run it on your own copy of the course.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("apply-solutions") if args.iter().any(|arg| arg == "--ci") => {
            match apply_solutions() {
                Ok((files, projects)) => {
                    println!("Applied {files} solution file(s) across {projects} project(s).");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("apply-solutions failed: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("apply-solutions") => {
            eprintln!(
                "apply-solutions overwrites your src/ files with the reference solutions.\n\
                 It is for CI only. If you really mean it, pass --ci."
            );
            ExitCode::FAILURE
        }
        _ => {
            eprintln!("usage: cargo xtask apply-solutions --ci");
            ExitCode::FAILURE
        }
    }
}

fn workspace_root() -> PathBuf {
    let xtask_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.parent().unwrap_or(xtask_dir).to_path_buf()
}

fn apply_solutions() -> io::Result<(usize, usize)> {
    let mut files = 0;
    let mut projects = 0;

    for entry in fs::read_dir(workspace_root())? {
        let project = entry?.path();
        let is_project = project
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("0x"));
        let solution_dir = project.join("solution");
        if !is_project || !solution_dir.is_dir() {
            continue;
        }

        projects += 1;
        for file in fs::read_dir(&solution_dir)? {
            let source = file?.path();
            if source.extension().is_some_and(|ext| ext == "rs") {
                if let Some(name) = source.file_name() {
                    fs::copy(&source, project.join("src").join(name))?;
                    files += 1;
                }
            }
        }
    }

    Ok((files, projects))
}
