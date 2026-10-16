//! 0x09. The rules engine: the capstone of the foundations.
//!
//! `model` is given. The tasks build on each other, in order:
//! t00 → when a holding expires, t01 → the state of one holding, t02 → the best holding for a
//! requirement (including equivalents), t03 → prerequisites, t04 → the gap check for a role.

pub mod model;
pub mod sample;

pub mod t00_valid_until;
pub mod t01_holding_state;
pub mod t02_best_holding;
pub mod t03_requirement_state;
pub mod t04_gap_check;
pub mod t100_validate_catalogue;
