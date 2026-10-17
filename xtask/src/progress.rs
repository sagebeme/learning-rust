//! `cargo xtask progress`: every task's state, from its tests and clippy. It never changes your
//! code. The same report is saved to `progress.json` for the course site.

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use crate::course::{self, Project};
use crate::run::{self, Build, TestResult};

/// Where a task stands. The words match the course site's.
#[derive(Clone, Copy, PartialEq)]
enum State {
    /// Tests pass and clippy has nothing to say.
    Done,
    /// Tests pass, but clippy has warnings.
    Lint,
    /// Some tests fail on something other than `todo!()`.
    Failing,
    /// The failing tests all stop at `todo!()`: not written yet.
    Todo,
    /// The tests need Postgres, and it isn't running.
    NeedsDb,
    /// The project doesn't compile, so no task in it can be checked.
    Broken,
}

impl State {
    fn key(self) -> &'static str {
        match self {
            State::Done => "done",
            State::Lint => "lint",
            State::Failing => "failing",
            State::Todo => "todo",
            State::NeedsDb => "needs-db",
            State::Broken => "broken",
        }
    }

    fn mark(self) -> &'static str {
        match self {
            State::Done => "[x]",
            State::Lint => "[~]",
            State::Failing => "[!]",
            State::Todo | State::NeedsDb | State::Broken => "[ ]",
        }
    }
}

pub fn run(filters: &[String]) -> Result<(), String> {
    let root = course::workspace_root();
    let projects: Vec<Project> = course::projects(&root)?
        .into_iter()
        .filter(|project| {
            filters.is_empty()
                || filters.iter().any(|filter| {
                    project.dir.starts_with(filter.as_str()) || project.package == *filter
                })
        })
        .collect();
    if projects.is_empty() {
        return Err(format!(
            "No project matches {}. Try `0x01` or `values-control`.",
            filters.join(" ")
        ));
    }

    println!(
        "Checking {} project(s). The first run builds everything, so it takes a few minutes.\n",
        projects.len()
    );
    let mut report = serde_json::Map::new();
    let (mut done, mut total, mut extra_done, mut extra_total) = (0, 0, 0, 0);
    let mut needs_db = false;

    for project in &projects {
        let build = run::test_project(&root, project, true)?;
        let mut tasks_json = serde_json::Map::new();
        let mut lines = Vec::new();
        let mut project_done = 0;

        for task in &project.tasks {
            let (state, passed, count, warnings) = match &build {
                Build::Failed(_) => (State::Broken, 0, 0, 0),
                Build::Compiled { tests, clippy } => {
                    let mine: Vec<&TestResult> = tests
                        .iter()
                        .filter(|test| {
                            run::task_of(&project.tasks, test).is_some_and(|t| t.id == task.id)
                        })
                        .collect();
                    let passed = mine.iter().filter(|test| test.passed).count();
                    let warnings = clippy.get(&task.file).copied().unwrap_or(0);
                    let failing: Vec<&&TestResult> =
                        mine.iter().filter(|test| !test.passed).collect();
                    let state = if mine.is_empty() {
                        State::Todo
                    } else if failing.is_empty() {
                        if warnings == 0 {
                            State::Done
                        } else {
                            State::Lint
                        }
                    } else if failing.iter().any(|test| test.needs_db) {
                        State::NeedsDb
                    } else if failing.iter().all(|test| test.todo) {
                        State::Todo
                    } else {
                        State::Failing
                    };
                    (state, passed, mine.len(), warnings)
                }
            };

            if state == State::Done {
                project_done += 1;
            }
            if task.advanced {
                extra_total += 1;
                extra_done += usize::from(state == State::Done);
            } else {
                total += 1;
                done += usize::from(state == State::Done);
            }
            needs_db |= state == State::NeedsDb;

            let note = match state {
                State::Done => String::new(),
                State::Lint => format!("tests pass, clippy has {warnings} warning(s)"),
                State::Failing => format!("{passed} of {count} tests pass"),
                State::Todo => "to do".to_string(),
                State::NeedsDb => "needs Postgres running (see the project README)".to_string(),
                State::Broken => "doesn't compile".to_string(),
            };
            let advanced = if task.advanced { " (advanced)" } else { "" };
            lines.push((
                format!(
                    "  {} {:<5} {}{advanced}",
                    state.mark(),
                    task.id,
                    plain(&task.title)
                ),
                note,
            ));
            tasks_json.insert(
                task.id.clone(),
                json!({ "state": state.key(), "passed": passed, "tests": count, "clippy": warnings }),
            );
        }

        println!(
            "{} {} — {project_done} of {} done",
            project.number,
            plain(&project.title),
            project.tasks.len()
        );
        let width = lines
            .iter()
            .map(|(task, _)| task.chars().count())
            .max()
            .unwrap_or(0);
        for (task, note) in lines {
            println!("{}", format!("{task:<width$}   {note}").trim_end());
        }
        if let Build::Failed(errors) = &build {
            println!("  It doesn't compile yet. The first error:\n");
            for line in first_error(errors).lines() {
                println!("    {line}");
            }
        }
        println!();

        report.insert(
            project.dir.clone(),
            json!({ "compiles": matches!(build, Build::Compiled { .. }), "tasks": Value::Object(tasks_json) }),
        );
    }

    println!("Mandatory tasks: {done} of {total} done. Advanced: {extra_done} of {extra_total}.");
    if needs_db {
        println!("Some tests need Postgres: start it as 0x0B's README shows, then run this again.");
    }

    let checked = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |time| time.as_secs());
    let file = root.join("progress.json");
    let saved = json!({ "checked": checked, "projects": Value::Object(report) });
    // Projects that weren't checked this time keep what the last run found.
    let merged = match fs::read_to_string(&file)
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
    {
        Some(mut old) if old["projects"].is_object() && !filters.is_empty() => {
            if let (Some(old_projects), Some(new_projects)) = (
                old["projects"].as_object_mut(),
                saved["projects"].as_object(),
            ) {
                for (dir, state) in new_projects {
                    old_projects.insert(dir.clone(), state.clone());
                }
            }
            old["checked"] = json!(checked);
            old
        }
        _ => saved,
    };
    let text = serde_json::to_string_pretty(&merged).map_err(|error| error.to_string())?;
    fs::write(&file, text + "\n").map_err(|error| format!("can't save progress.json: {error}"))?;
    println!(
        "Saved to progress.json: the course site shows it when you run it locally (see the README)."
    );
    Ok(())
}

/// Titles keep their Markdown backticks for the site; the terminal doesn't need them.
fn plain(title: &str) -> String {
    title.replace('`', "")
}

/// The compiler's first error and its explanation, up to the next blank-line-separated message.
fn first_error(errors: &str) -> String {
    let start = errors
        .find("error[")
        .or_else(|| errors.find("error:"))
        .unwrap_or(0);
    errors[start..]
        .split("\n\n")
        .next()
        .unwrap_or_default()
        .to_string()
}
