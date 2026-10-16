//! The reference TWA engine: the 0x09 reference solution, packaged as a library.
//!
//! Weeks 5 to 8 use the engine. By default they use this one, so a gap in your own 0x09 never
//! blocks the later weeks. Once your 0x09 passes its tests, switch each later project to your own
//! engine: its `Cargo.toml` shows the one line to change.
//!
//! There's no copy of the code here: each module below *is* the file from 0x09, pulled in with
//! `#[path]`, so the reference can never drift from the solution. The module names match your
//! 0x09 crate exactly, which is what makes the switch a one-line change.

#[path = "../../0x09-rules-engine/src/model.rs"]
pub mod model;
#[path = "../../0x09-rules-engine/src/sample.rs"]
pub mod sample;

#[path = "../../0x09-rules-engine/solution/t00_valid_until.rs"]
pub mod t00_valid_until;
#[path = "../../0x09-rules-engine/solution/t01_holding_state.rs"]
pub mod t01_holding_state;
#[path = "../../0x09-rules-engine/solution/t02_best_holding.rs"]
pub mod t02_best_holding;
#[path = "../../0x09-rules-engine/solution/t03_requirement_state.rs"]
pub mod t03_requirement_state;
#[path = "../../0x09-rules-engine/solution/t04_gap_check.rs"]
pub mod t04_gap_check;
#[path = "../../0x09-rules-engine/solution/t100_validate_catalogue.rs"]
pub mod t100_validate_catalogue;
