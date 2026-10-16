//! Task 2: state shared by every command.
//!
//! Tauri can run commands at the same time, so the staff list lives behind a `Mutex`.
//! - `lock_staff`: lock it, turning a poisoned lock into `CommandError::Busy` (never panic)
//! - `staff_summary`: the summary for one person, or `NotFound("staff member <number>")`
//! - `add_holding`: record a qualification for a person (or `NotFound`). If they already hold
//!   that code, keep the **later** completion date. Return their updated summary.
//!
//! A summary lists the codes held, sorted, and serializes with camelCase keys for the screens.

use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};

use chrono::NaiveDate;
use serde::Serialize;

use crate::t00_command_error::CommandError;
use crate::t01_inputs::NewHolding;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaffMember {
    pub staff_number: u32,
    pub name: String,
    /// code → completion date
    pub holdings: BTreeMap<String, NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffSummary {
    pub staff_number: u32,
    pub name: String,
    pub codes: Vec<String>,
}

#[derive(Debug, Default)]
pub struct AppState {
    staff: Mutex<BTreeMap<u32, StaffMember>>,
}

// Given: helpers for setting up and testing.
impl AppState {
    pub fn with_staff(staff: &[(u32, &str)]) -> AppState {
        let staff = staff
            .iter()
            .map(|&(staff_number, name)| {
                let member = StaffMember {
                    staff_number,
                    name: name.to_string(),
                    holdings: BTreeMap::new(),
                };
                (staff_number, member)
            })
            .collect();
        AppState {
            staff: Mutex::new(staff),
        }
    }

    /// Poisons the lock, the way a command that panicked mid-update would. For tests only.
    pub fn poison_for_test(&self) {
        let _ = std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    let _guard = self.staff.lock();
                    panic!("a command panicked while holding the lock");
                })
                .join()
        });
    }
}

impl AppState {
    pub fn lock_staff(&self) -> Result<MutexGuard<'_, BTreeMap<u32, StaffMember>>, CommandError> {
        self.staff.lock().map_err(|_| CommandError::Busy)
    }
}

fn summary_of(member: &StaffMember) -> StaffSummary {
    StaffSummary {
        staff_number: member.staff_number,
        name: member.name.clone(),
        codes: member.holdings.keys().cloned().collect(),
    }
}

fn not_found(staff_number: u32) -> CommandError {
    CommandError::NotFound(format!("staff member {staff_number}"))
}

pub fn staff_summary(state: &AppState, staff_number: u32) -> Result<StaffSummary, CommandError> {
    let staff = state.lock_staff()?;
    staff
        .get(&staff_number)
        .map(summary_of)
        .ok_or_else(|| not_found(staff_number))
}

pub fn add_holding(state: &AppState, input: NewHolding) -> Result<StaffSummary, CommandError> {
    let mut staff = state.lock_staff()?;
    let member = staff
        .get_mut(&input.staff_number)
        .ok_or_else(|| not_found(input.staff_number))?;
    let completed_on = member
        .holdings
        .entry(input.code)
        .or_insert(input.completed_on);
    if input.completed_on > *completed_on {
        *completed_on = input.completed_on;
    }
    Ok(summary_of(member))
}
