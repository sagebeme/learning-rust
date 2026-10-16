//! Task 2: state shared by every command.
//!
//! Tauri can run commands at the same time, so the staff list lives behind a `Mutex`.
//! - `lock_staff`: lock it, turning a poisoned lock into `CommandError::Busy` (never panic)
//! - `staff_summary`: the summary for one person, or `NotFound("staff member <number>")`
//! - `add_holding`: record a qualification for a person (or `NotFound`). If they already hold
//!   that code, keep the **later** completion date. Return their updated summary.
//!
//! A summary lists the codes held, sorted, and serializes with camelCase keys for the screens.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

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
        todo!()
    }
}

pub fn staff_summary(state: &AppState, staff_number: u32) -> Result<StaffSummary, CommandError> {
    todo!()
}

pub fn add_holding(state: &AppState, input: NewHolding) -> Result<StaffSummary, CommandError> {
    todo!()
}
