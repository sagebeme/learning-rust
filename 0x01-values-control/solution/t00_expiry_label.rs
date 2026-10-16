//! Task 0: what does a licence's countdown mean?
//!
//! Given how many days are left until a licence expires:
//! - fewer than 0: `"expired"`
//! - exactly 0: `"expires today"`
//! - 1 to 30: `"expiring soon"`
//! - more than 30: `"valid"`

pub fn expiry_label(days_left: i64) -> &'static str {
    match days_left {
        ..0 => "expired",
        0 => "expires today",
        1..=30 => "expiring soon",
        _ => "valid",
    }
}
