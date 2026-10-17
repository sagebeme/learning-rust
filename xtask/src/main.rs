//! Course chores. Run with `cargo xtask <command>`.
//!
//! For you:
//! - `progress` runs every project's tests and clippy, and shows which tasks are done. It also
//!   saves `progress.json`, which the course site reads.
//! - `verify-solutions` proves the reference solutions pass, in a copy of the course, so your
//!   own `src/` is never touched.
//!
//! For CI:
//! - `check-starters` proves every task's tests fail against its empty starter, so no task can
//!   pass before you've written anything.
//! - `site-data` rebuilds the project list the course site uses (`--check` fails if it's stale).
//! - `apply-solutions --ci` copies every project's `solution/*.rs` over the matching `src/*.rs`,
//!   so CI can prove that every reference solution passes its tests. It overwrites your work, so
//!   it refuses to run without `--ci`. Never run it on your own copy of the course.

mod checks;
mod course;
mod progress;
mod run;
mod site;

use std::env;
use std::process::ExitCode;

const USAGE: &str = "usage:
  cargo xtask progress [project...]   which tasks are done (e.g. `progress 0x01 0x02`)
  cargo xtask verify-solutions        the reference solutions pass, checked in a copy
  cargo xtask check-starters          every task's tests fail on its starter (CI)
  cargo xtask site-data [--check]     rebuild the course site's project list (CI checks it)
  cargo xtask apply-solutions --ci    overwrite src/ with the solutions (CI only)";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let rest = args.get(1..).unwrap_or_default();
    let result = match args.first().map(String::as_str) {
        Some("progress") => progress::run(rest),
        Some("verify-solutions") => checks::verify_solutions(),
        Some("check-starters") => checks::check_starters(),
        Some("site-data") => site::run(rest.iter().any(|arg| arg == "--check")),
        Some("apply-solutions") if rest.iter().any(|arg| arg == "--ci") => {
            checks::apply_solutions(&course::workspace_root()).map(|(files, projects)| {
                println!("Applied {files} solution file(s) across {projects} project(s).");
            })
        }
        Some("apply-solutions") => Err(
            "apply-solutions overwrites your src/ files with the reference solutions.\n\
             It is for CI only. If you really mean it, pass --ci.\n\
             To check the solutions without touching your work, run `cargo xtask verify-solutions`."
                .to_string(),
        ),
        _ => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
