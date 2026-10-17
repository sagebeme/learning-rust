//! Builds a project's tests, runs them one program at a time, and says which task each test
//! belongs to. `progress` and `check-starters` both read the course through this.

use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::course::{Project, Task};

pub struct TestResult {
    /// The test program, e.g. `t00_expiry_label` (a file in `tests/`) or `values_control` (the
    /// unit tests in `src/`).
    pub target: String,
    pub name: String,
    pub passed: bool,
    /// It failed on a `todo!()`: the starter hasn't been filled in.
    pub todo: bool,
    /// It failed because there's no Postgres to talk to.
    pub needs_db: bool,
}

pub enum Build {
    Compiled {
        tests: Vec<TestResult>,
        /// Clippy warnings by file name, e.g. `t00_expiry_label.rs` → 2.
        clippy: BTreeMap<String, usize>,
    },
    /// The compiler's errors, as it printed them.
    Failed(String),
}

/// Builds and runs every test in `project`. Clippy runs too when `with_clippy` is set.
pub fn test_project(root: &Path, project: &Project, with_clippy: bool) -> Result<Build, String> {
    let output = cargo(root)
        .args([
            "test",
            "--no-run",
            "--message-format=json-render-diagnostics",
            "-p",
        ])
        .arg(&project.package)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("can't run cargo: {error}"))?;
    if !output.status.success() {
        return Ok(Build::Failed(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }

    let mut tests = Vec::new();
    for (target, executable) in test_programs(&output.stdout) {
        let run = Command::new(&executable)
            .current_dir(&project.path)
            .env("CARGO_MANIFEST_DIR", &project.path)
            .env("CARGO_PKG_NAME", &project.package)
            .output()
            .map_err(|error| format!("can't run {}: {error}", executable.display()))?;
        let stdout = String::from_utf8_lossy(&run.stdout);
        let mut results = parse_results(&target, &stdout);
        if !run.status.success() && results.iter().all(|result| result.passed) {
            // It stopped without a report (a crash, or a failure before any test ran).
            let stderr = String::from_utf8_lossy(&run.stderr);
            results.push(TestResult {
                name: format!("{target} (the test program stopped early)"),
                target: target.clone(),
                passed: false,
                todo: stderr.contains("not yet implemented"),
                needs_db: mentions_database(&stderr),
            });
        }
        tests.extend(results);
    }

    let clippy = if with_clippy {
        clippy_warnings(root, project)?
    } else {
        BTreeMap::new()
    };
    Ok(Build::Compiled { tests, clippy })
}

/// Cargo, as the one that started xtask, run from the workspace.
fn cargo(root: &Path) -> Command {
    let mut command = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(root);
    command
}

/// The test programs Cargo just built: (target name, executable).
fn test_programs(json_lines: &[u8]) -> Vec<(String, PathBuf)> {
    String::from_utf8_lossy(json_lines)
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|message| {
            message["reason"] == "compiler-artifact" && message["profile"]["test"] == true
        })
        .filter_map(|message| {
            let target = message["target"]["name"].as_str()?.to_string();
            let executable = message["executable"].as_str()?;
            Some((target, PathBuf::from(executable)))
        })
        .collect()
}

/// Reads libtest's report: `test name ... ok`, then each failure's output under `---- name stdout ----`.
fn parse_results(target: &str, stdout: &str) -> Vec<TestResult> {
    let mut failure_output: BTreeMap<&str, String> = BTreeMap::new();
    let mut current: Option<&str> = None;
    for line in stdout.lines() {
        if let Some(name) = line
            .strip_prefix("---- ")
            .and_then(|rest| rest.strip_suffix(" stdout ----"))
        {
            current = Some(name);
        } else if line == "failures:" || line.starts_with("test result:") {
            current = None;
        } else if let Some(name) = current {
            let text = failure_output.entry(name).or_default();
            text.push_str(line);
            text.push('\n');
        }
    }

    stdout
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("test ")?;
            let (name, outcome) = rest.rsplit_once(" ... ")?;
            let passed = match outcome {
                "ok" => true,
                "FAILED" => false,
                _ => return None, // ignored, or a benchmark
            };
            let output = failure_output.get(name).map_or("", String::as_str);
            Some(TestResult {
                target: target.to_string(),
                name: name.to_string(),
                passed,
                todo: !passed && output.contains("not yet implemented"),
                needs_db: !passed && mentions_database(output),
            })
        })
        .collect()
}

fn mentions_database(output: &str) -> bool {
    [
        "DATABASE_URL",
        "Connection refused",
        "connection refused",
        "communicating with database",
        "PoolTimedOut",
    ]
    .iter()
    .any(|sign| output.contains(sign))
}

/// Which task a test checks:
/// - every test in `tests/t00_expiry_label.rs`
/// - unit tests inside `src/t00_expiry_label.rs` (`t00_expiry_label::tests::...`)
/// - any test whose name starts with the task's id, like `t00_inserts_an_employee`
///
/// Anything else (`conformance.rs`, `this_repo.rs`) checks the project as a whole.
pub fn task_of<'a>(tasks: &'a [Task], test: &TestResult) -> Option<&'a Task> {
    let function = test.name.rsplit("::").next().unwrap_or(&test.name);
    tasks.iter().find(|task| {
        test.target == task.stem()
            || test.name.starts_with(&format!("{}::", task.stem()))
            || function.starts_with(&format!("{}_", task.id))
    })
}

/// `cargo clippy -p <package>` on the project's own code, counted by file.
fn clippy_warnings(root: &Path, project: &Project) -> Result<BTreeMap<String, usize>, String> {
    let output = cargo(root)
        .args(["clippy", "--message-format=json", "-p"])
        .arg(&project.package)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|error| format!("can't run cargo clippy: {error}"))?;
    let mut warnings = BTreeMap::new();
    let own_files = format!("{}/src/", project.dir);
    for message in String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|message| message["reason"] == "compiler-message")
    {
        let spans = message["message"]["spans"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let file = spans
            .iter()
            .filter(|span| span["is_primary"] == true)
            .find_map(|span| span["file_name"].as_str())
            .map(|name| name.replace('\\', "/"));
        if let Some(file) = file
            && let Some(name) = file.strip_prefix(&own_files)
        {
            *warnings.entry(name.to_string()).or_insert(0) += 1;
        }
    }
    Ok(warnings)
}
