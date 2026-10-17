//! What the course is made of, read from the folders: projects, their tasks, and the README
//! details the site shows.

use std::fs;
use std::path::{Path, PathBuf};

pub struct Project {
    /// The folder, e.g. `0x01-values-control`.
    pub dir: String,
    pub path: PathBuf,
    /// The Cargo package, e.g. `values-control`.
    pub package: String,
    /// `0x01`
    pub number: String,
    /// From the README's heading: `Values and control flow`.
    pub title: String,
    pub week: u32,
    /// The README's one-line summary, after "**Week 1** ·".
    pub summary: String,
    pub tasks: Vec<Task>,
    pub has_review: bool,
}

pub struct Task {
    /// `t00`, `t01`, `t100`: the start of the file name, and of any test named after the task.
    pub id: String,
    pub number: u32,
    /// `t00_expiry_label.rs`
    pub file: String,
    /// From the README's `### 0. Title (`file`)` heading, or the file name if there isn't one.
    pub title: String,
    /// Tasks numbered 100 and up.
    pub advanced: bool,
}

impl Task {
    /// `t00_expiry_label`: the name of the task's test file and of its module.
    pub fn stem(&self) -> &str {
        self.file.trim_end_matches(".rs")
    }
}

pub fn workspace_root() -> PathBuf {
    let xtask_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.parent().unwrap_or(xtask_dir).to_path_buf()
}

/// Every `0x..` project under `root`, in order.
pub fn projects(root: &Path) -> Result<Vec<Project>, String> {
    let mut dirs: Vec<String> = fs::read_dir(root)
        .map_err(|error| format!("can't list {}: {error}", root.display()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().join("Cargo.toml").is_file())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
        .filter(|name| name.starts_with("0x"))
        .collect();
    dirs.sort();
    dirs.iter().map(|dir| project(root, dir)).collect()
}

fn project(root: &Path, dir: &str) -> Result<Project, String> {
    let path = root.join(dir);
    let read = |file: &str| {
        fs::read_to_string(path.join(file))
            .map_err(|error| format!("can't read {dir}/{file}: {error}"))
    };
    let manifest = read("Cargo.toml")?;
    let readme = read("README.md")?;

    let package = manifest
        .lines()
        .find_map(|line| line.strip_prefix("name = \""))
        .and_then(|rest| rest.strip_suffix('"'))
        .ok_or_else(|| format!("{dir}/Cargo.toml has no package name"))?
        .to_string();

    // "# 0x01. Values and control flow"
    let heading = readme
        .lines()
        .find_map(|line| line.strip_prefix("# "))
        .ok_or_else(|| format!("{dir}/README.md has no heading"))?;
    let (number, title) = heading
        .split_once(". ")
        .ok_or_else(|| format!("{dir}/README.md's heading isn't \"0x00. Title\""))?;

    // "**Week 1** · Tooling, your first functions, and a first look at memory."
    let (week, summary) = readme
        .lines()
        .find_map(|line| {
            let rest = line.strip_prefix("**Week ")?;
            let (week, summary) = rest.split_once("** · ")?;
            Some((week.parse().ok()?, summary.to_string()))
        })
        .ok_or_else(|| format!("{dir}/README.md has no \"**Week N** · summary\" line"))?;

    let mut tasks = Vec::new();
    let src = path.join("src");
    for entry in fs::read_dir(&src).map_err(|error| format!("can't list {dir}/src: {error}"))? {
        let Some(file) = entry
            .ok()
            .and_then(|e| e.file_name().to_str().map(str::to_string))
        else {
            continue;
        };
        let Some(number) = task_number(&file) else {
            continue;
        };
        let id = file.split('_').next().unwrap_or_default().to_string();
        let (title, advanced) = task_heading(&readme, &file)
            .unwrap_or_else(|| (file.trim_end_matches(".rs").to_string(), number >= 100));
        tasks.push(Task {
            id,
            number,
            file,
            title,
            advanced,
        });
    }
    tasks.sort_by_key(|task| task.number);

    Ok(Project {
        dir: dir.to_string(),
        package,
        number: number.to_string(),
        title: title.trim().to_string(),
        week,
        summary,
        tasks,
        has_review: path.join("review").is_dir(),
        path,
    })
}

/// `t04_duty_board.rs` is task 4. Anything else in `src/` is not a task.
fn task_number(file: &str) -> Option<u32> {
    let stem = file.strip_suffix(".rs")?;
    let (id, _) = stem.split_once('_')?;
    id.strip_prefix('t')?.parse().ok()
}

/// From `### 100. Minutes as a clock reading (`t100_minutes_to_clock.rs`) — advanced`.
fn task_heading(readme: &str, file: &str) -> Option<(String, bool)> {
    let marker = format!(" (`{file}`)");
    readme.lines().find_map(|line| {
        let heading = line.strip_prefix("### ")?;
        let (before, after) = heading.split_once(&marker)?;
        let title = before.split_once(". ").map_or(before, |(_, title)| title);
        Some((title.to_string(), after.contains("advanced")))
    })
}
