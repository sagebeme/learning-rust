//! 0x0A. The Rust core of the TWA Tauri app. Each task is one module; the `#[tauri::command]`
//! wrappers in the real app call these functions. Do the tasks in order.

pub mod t00_command_error;
pub mod t01_inputs;
pub mod t02_app_state;
pub mod t03_capabilities;
pub mod t100_events;
