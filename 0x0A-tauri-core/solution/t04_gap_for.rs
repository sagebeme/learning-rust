//! Task 4: the gap check, as a command the app's screens can call.
//!
//! This is where the app meets the engine you built in 0x09. `gap_for` answers "can this person
//! start in this role on this day?" for the screens:
//! 1. lock the staff list; an unknown staff number → `NotFound("staff member <number>")`
//! 2. turn their holdings (code → completion date) into the engine's `Holding`s
//!    (`last_used_on: None`: the app doesn't record use yet)
//! 3. call `twa_engine::t04_gap_check::gap_check` with the state's catalogue and matrix
//! 4. turn engine errors into command errors: `UnknownRole` →
//!    `Invalid { field: "role", reason: <the engine error's message> }`, and
//!    `UnknownQualification` → `Internal(<the message>)`, because that's a broken catalogue,
//!    not something the person using the app can fix
//!
//! Let go of the lock before calling the engine: it only needs a copy of the holdings.

use chrono::NaiveDate;
use twa_engine::model::{EngineError, GapReport, Holding};
use twa_engine::t04_gap_check::gap_check;

use crate::t00_command_error::CommandError;
use crate::t02_app_state::AppState;

pub fn gap_for(
    state: &AppState,
    staff_number: u32,
    department: &str,
    role: &str,
    today: NaiveDate,
) -> Result<GapReport, CommandError> {
    let holdings: Vec<Holding> = {
        let staff = state.lock_staff()?;
        let member = staff
            .get(&staff_number)
            .ok_or_else(|| CommandError::NotFound(format!("staff member {staff_number}")))?;
        member
            .holdings
            .iter()
            .map(|(code, completed_on)| Holding {
                code: code.clone(),
                completed_on: *completed_on,
                last_used_on: None,
            })
            .collect()
    }; // the lock is released here

    gap_check(
        state.catalogue(),
        state.matrix(),
        department,
        role,
        &holdings,
        today,
    )
    .map_err(|error| match error {
        EngineError::UnknownRole { .. } => CommandError::Invalid {
            field: "role".to_string(),
            reason: error.to_string(),
        },
        EngineError::UnknownQualification(_) => CommandError::Internal(error.to_string()),
    })
}
