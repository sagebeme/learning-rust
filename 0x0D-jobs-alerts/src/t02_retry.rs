//! Task 2: try again, politely.
//!
//! Send the alert. On a `Transient` error, wait and try again: `first_delay` before the second
//! attempt, then double the wait each time (1s, 2s, 4s, ...), up to `max_attempts` attempts in
//! total (treat 0 as 1). On a `Permanent` error, stop at once. Return how many attempts it took
//! on success, or the last error.
//!
//! Wait with `tokio::time::sleep`, never `std::thread::sleep`.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

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
    todo!()
}
