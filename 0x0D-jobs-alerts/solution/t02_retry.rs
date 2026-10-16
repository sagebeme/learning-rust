//! Task 2: try again, politely.
//!
//! Send the alert. On a `Transient` error, wait and try again: `first_delay` before the second
//! attempt, then double the wait each time (1s, 2s, 4s, ...), up to `max_attempts` attempts in
//! total (treat 0 as 1). On a `Permanent` error, stop at once. Return how many attempts it took
//! on success, or the last error.
//!
//! Wait with `tokio::time::sleep`, never `std::thread::sleep`.

use std::time::Duration;

use crate::model::{Alert, AlertSender, SendError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub first_delay: Duration,
}

pub async fn send_with_retry<S: AlertSender>(
    sender: &S,
    alert: &Alert,
    policy: RetryPolicy,
) -> Result<u32, SendError> {
    let max_attempts = policy.max_attempts.max(1);
    let mut delay = policy.first_delay;
    let mut attempt = 1;
    loop {
        match sender.send(alert).await {
            Ok(()) => return Ok(attempt),
            Err(error @ SendError::Permanent(_)) => return Err(error),
            Err(error) if attempt >= max_attempts => return Err(error),
            Err(SendError::Transient(_)) => {
                tokio::time::sleep(delay).await;
                delay = delay.saturating_mul(2);
                attempt += 1;
            }
        }
    }
}
