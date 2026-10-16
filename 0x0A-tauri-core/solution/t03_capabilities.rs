//! Task 3: review a window's permissions automatically.
//!
//! This is a simplified version of a Tauri 2 capabilities file:
//! `{"identifier": "main", "windows": ["main"], "permissions": ["core:default", "allow-add-holding"]}`
//!
//! `check_capability(json, commands)` returns every problem it finds, sorted, each once:
//! - `NoWindows` if `windows` is empty; `AllWindows` if it contains `"*"`
//! - `MissingPermission(command)` for each registered command without an `allow-<name>`
//!   permission, where `<name>` is the command with `_` turned into `-`
//!   (`add_holding` needs `allow-add-holding`)
//! - `UnknownCommandPermission(permission)` for each `allow-...` permission (no `:` in it) that
//!   doesn't match a registered command
//! - `Dangerous(permission)` for each permission in `DANGEROUS`
//!
//! JSON that isn't a valid capability → `CommandError::Invalid { field: "capability", .. }`.

use std::collections::BTreeSet;

use serde::Deserialize;

use crate::t00_command_error::CommandError;

/// Plugin permissions that let screen code run programs on the device.
pub const DANGEROUS: [&str; 2] = ["shell:allow-execute", "shell:allow-spawn"];

#[derive(Debug, Deserialize)]
pub struct Capability {
    pub identifier: String,
    pub windows: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Problem {
    NoWindows,
    AllWindows,
    MissingPermission(String),
    UnknownCommandPermission(String),
    Dangerous(String),
}

pub fn check_capability(json: &str, commands: &[&str]) -> Result<Vec<Problem>, CommandError> {
    let capability: Capability =
        serde_json::from_str(json).map_err(|error| CommandError::Invalid {
            field: "capability".to_string(),
            reason: error.to_string(),
        })?;

    let mut problems = BTreeSet::new();
    if capability.windows.is_empty() {
        problems.insert(Problem::NoWindows);
    }
    if capability.windows.iter().any(|w| w == "*") {
        problems.insert(Problem::AllWindows);
    }

    let granted: BTreeSet<&str> = capability.permissions.iter().map(String::as_str).collect();
    let permission_for = |command: &str| format!("allow-{}", command.replace('_', "-"));
    let expected: BTreeSet<String> = commands.iter().map(|c| permission_for(c)).collect();

    for command in commands {
        if !granted.contains(permission_for(command).as_str()) {
            problems.insert(Problem::MissingPermission(command.to_string()));
        }
    }

    for permission in &capability.permissions {
        if permission.starts_with("allow-")
            && !permission.contains(':')
            && !expected.contains(permission)
        {
            problems.insert(Problem::UnknownCommandPermission(permission.clone()));
        }
        if DANGEROUS.contains(&permission.as_str()) {
            problems.insert(Problem::Dangerous(permission.clone()));
        }
    }

    Ok(problems.into_iter().collect())
}
