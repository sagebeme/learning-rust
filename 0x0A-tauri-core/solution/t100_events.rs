//! Task 100 (advanced): events from the core to the screens.
//!
//! - `expiring_payload` builds what the core sends when a qualification is about to expire, with
//!   a `message` for the screen: `"DG expires in 3 days"`, `"DG expires tomorrow"`,
//!   `"DG expires today"`, `"DG expired yesterday"`, `"DG expired 4 days ago"`
//! - `is_valid_event_name`: Tauri only accepts event names made of letters, digits, `-`, `/`, `:`
//!   and `_`, and not empty. Check a name before using it, so a typo fails a test, not the app.

use serde::Serialize;

pub const EXPIRING_EVENT: &str = "qualification-expiring";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpiringPayload {
    pub staff_number: u32,
    pub code: String,
    pub days_left: i64,
    pub message: String,
}

pub fn expiring_payload(staff_number: u32, code: &str, days_left: i64) -> ExpiringPayload {
    let when = match days_left {
        0 => "expires today".to_string(),
        1 => "expires tomorrow".to_string(),
        -1 => "expired yesterday".to_string(),
        days if days > 1 => format!("expires in {days} days"),
        days => format!("expired {} days ago", days.unsigned_abs()),
    };
    ExpiringPayload {
        staff_number,
        code: code.to_string(),
        days_left,
        message: format!("{code} {when}"),
    }
}

pub fn is_valid_event_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '/' | ':' | '_'))
}
