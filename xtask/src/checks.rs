//! Checks on the course itself: the starters, the solutions, and the checker between them.

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::course::{self, Project};
use crate::run::{self, Build};

/// Copies every project's `solution/*.rs` over the matching `src/*.rs`. Returns (files, projects).
pub fn apply_solutions(root: &Path) -> Result<(usize, usize), String> {
    let mut files = 0;
    let mut projects = 0;
    for project in course::projects(root)? {
        let solution_dir = project.path.join("solution");
        if !solution_dir.is_dir() {
            continue;
        }
        projects += 1;
        for file in fs::read_dir(&solution_dir).map_err(|error| error.to_string())? {
            let source = file.map_err(|error| error.to_string())?.path();
            if source.extension().is_some_and(|ext| ext == "rs")
                && let Some(name) = source.file_name()
            {
                // Read and write rather than fs::copy: on Windows, copying keeps the old file's
                // modified time, so Cargo would think nothing changed and reuse a stale build.
                let code = fs::read(&source).map_err(|error| error.to_string())?;
                fs::write(project.path.join("src").join(name), code)
                    .map_err(|error| error.to_string())?;
                files += 1;
            }
        }
    }
    Ok((files, projects))
}

/// For CI, on the course as published: every task has a solution and tests, the starters compile,
/// and every task has at least one test that fails on its starter. A task whose tests all pass
/// before anyone has written anything is a checker that checks nothing.
pub fn check_starters() -> Result<(), String> {
    let root = course::workspace_root();
    let mut problems = Vec::new();
    let mut checked = 0;

    for project in course::projects(&root)? {
        problems.extend(layout_problems(&project));
        match run::test_project(&root, &project, false)? {
            Build::Failed(errors) => problems.push(format!(
                "{}: the starter doesn't compile\n{errors}",
                project.dir
            )),
            Build::Compiled { tests, .. } => {
                for task in &project.tasks {
                    let mine: Vec<_> = tests
                        .iter()
                        .filter(|test| {
                            run::task_of(&project.tasks, test).is_some_and(|t| t.id == task.id)
                        })
                        .collect();
                    let where_ = format!("{}/src/{}", project.dir, task.file);
                    if mine.is_empty() {
                        problems.push(format!("{where_}: no test checks this task"));
                    } else if mine.iter().all(|test| test.passed) {
                        problems.push(format!(
                            "{where_}: all {} of its tests pass on the starter",
                            mine.len()
                        ));
                    } else if mine.iter().any(|test| test.needs_db) {
                        problems.push(format!(
                            "{where_}: its tests need Postgres (set DATABASE_URL)"
                        ));
                    }
                    checked += 1;
                }
            }
        }
        println!("checked {}", project.dir);
    }

    if problems.is_empty() {
        println!(
            "All {checked} tasks have tests, a solution, and at least one test that fails on the starter."
        );
        Ok(())
    } else {
        Err(format!(
            "{} problem(s):\n- {}",
            problems.len(),
            problems.join("\n- ")
        ))
    }
}

/// Every task in `src/` has a solution, and every solution replaces a file that exists in `src/`.
fn layout_problems(project: &Project) -> Vec<String> {
    let mut problems = Vec::new();
    for task in &project.tasks {
        if !project.path.join("solution").join(&task.file).is_file() {
            problems.push(format!(
                "{}/src/{}: no matching solution/{}",
                project.dir, task.file, task.file
            ));
        }
    }
    if let Ok(entries) = fs::read_dir(project.path.join("solution")) {
        for name in entries.filter_map(|entry| entry.ok()?.file_name().into_string().ok()) {
            if name.ends_with(".rs") && !project.path.join("src").join(&name).is_file() {
                problems.push(format!(
                    "{}/solution/{name}: there's no src/{name} for it to replace",
                    project.dir
                ));
            }
        }
    }
    problems
}

/// The CI checks on the solutions, run in a copy of the course under `target/`, so your own
/// `src/` is never touched.
pub fn verify_solutions() -> Result<(), String> {
    let root = course::workspace_root();
    let scratch = root.join("target").join("verify-solutions");
    let copy = scratch.join("course");
    if copy.exists() {
        fs::remove_dir_all(&copy)
            .map_err(|error| format!("can't clear {}: {error}", copy.display()))?;
    }
    copy_course(&root, &copy).map_err(|error| format!("can't copy the course: {error}"))?;
    let (files, projects) = apply_solutions(&copy)?;
    println!(
        "Copied the course to {} and applied {files} solution file(s) across {projects} project(s).",
        copy.display()
    );
    if env::var_os("DATABASE_URL").is_none() {
        println!(
            "DATABASE_URL isn't set, so 0x0B's tests will fail. Start Postgres as 0x0B's README shows."
        );
    }

    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let steps: [(&str, &[&str]); 3] = [
        ("Solutions are formatted", &["fmt", "--all", "--check"]),
        (
            "Clippy, warnings are errors",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("Tests", &["test", "--workspace"]),
    ];
    for (name, args) in steps {
        println!("\n== {name}: cargo {}", args.join(" "));
        let status = Command::new(&cargo)
            .args(args)
            .current_dir(&copy)
            .env("CARGO_TARGET_DIR", scratch.join("target"))
            .status()
            .map_err(|error| format!("can't run cargo: {error}"))?;
        if !status.success() {
            return Err(format!(
                "\n{name}: failed. The copy is in {}.",
                copy.display()
            ));
        }
    }
    println!(
        "\nEvery solution is formatted, passes clippy, and passes its tests. Your src/ wasn't touched."
    );
    Ok(())
}

/// The course's files, without builds, git history or your saved progress.
fn copy_course(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if matches!(
            name.to_str(),
            Some("target" | ".git" | "node_modules" | "progress.json")
        ) {
            continue;
        }
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            copy_course(&path, &to.join(&name))?;
        } else {
            fs::write(to.join(&name), fs::read(&path)?)?;
        }
    }
    Ok(())
}
