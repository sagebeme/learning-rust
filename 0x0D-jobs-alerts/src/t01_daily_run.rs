//! Task 1: run the day's alerts so that running twice never sends twice.
//!
//! - `dedupe_key(alert)`: a string that's the same for "the same alert", whatever day the job
//!   runs: `"<staff>:<code>:<valid_until>:<days_left>:<channel>"`, with the channel as
//!   `Email`, `Sms`, `WhatsApp` or `Push` (its `Debug` text), for example
//!   `"48213:DG:2026-11-15:30:Sms"`.
//! - `run_daily`: for each alert, skip it if the log says it was sent; otherwise send it. Mark it
//!   sent **only if sending worked**, so a failure is tried again on the next run. Send one at a
//!   time, in order. Count what happened.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use crate::model::{Alert, AlertSender, SentLog};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RunSummary {
    pub sent: usize,
    pub skipped: usize,
    pub failed: usize,
}

pub fn dedupe_key(alert: &Alert) -> String {
    todo!()
}

pub async fn run_daily<S: AlertSender, L: SentLog>(
    alerts: &[Alert],
    sender: &S,
    log: &mut L,
) -> RunSummary {
    todo!()
}
