//! Given: the types the alert job works with, and the trait every sender implements.

use std::collections::BTreeSet;
use std::future::Future;

use chrono::NaiveDate;

/// How an alert reaches someone. The order here is the order alerts are listed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    Email,
    Sms,
    WhatsApp,
    Push,
}

/// How to reach one staff member. Fake data only in this course.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub staff_number: u32,
    pub email: Option<String>,
    pub phone: Option<String>,
    /// They've agreed to receive WhatsApp messages on `phone` instead of SMS.
    pub whatsapp_opt_in: bool,
    pub push_token: Option<String>,
}

/// One qualification and when it expires, as found by the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpiringItem {
    pub staff_number: u32,
    pub code: String,
    pub valid_until: NaiveDate,
}

/// One message to send, on one channel, to one address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alert {
    pub staff_number: u32,
    pub code: String,
    pub valid_until: NaiveDate,
    pub days_left: i64,
    pub channel: Channel,
    /// The email address, phone number or push token.
    pub to: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SendError {
    /// Worth trying again: a timeout, the provider is busy, no network.
    #[error("temporary failure: {0}")]
    Transient(String),
    /// Trying again won't help: a bad number, an unsubscribed address.
    #[error("permanent failure: {0}")]
    Permanent(String),
}

/// Anything that can deliver an alert: SMTP, Africa's Talking, the WhatsApp Cloud API, push.
///
/// The method is written as "returns a future that is `Send`" rather than `async fn`, so that
/// code using this trait can spawn the work onto other threads. Implementations can still be
/// written with `async fn send(...)`.
pub trait AlertSender {
    fn send(&self, alert: &Alert) -> impl Future<Output = Result<(), SendError>> + Send;
}

/// Days before expiry (or after, if negative) on which a reminder goes out.
pub const REMINDER_DAYS: [i64; 6] = [60, 30, 7, 1, 0, -1];

/// The message for a reminder.
pub fn message_for(code: &str, days_left: i64) -> String {
    match days_left {
        0 => format!("Your {code} expires today."),
        1 => format!("Your {code} expires tomorrow."),
        -1 => format!("Your {code} expired yesterday. Please book a refresher."),
        days if days < 0 => format!("Your {code} expired {} days ago.", -days),
        days => format!("Your {code} expires in {days} days."),
    }
}

/// Remembers which alerts were already sent, so running the job twice sends nothing twice.
pub trait SentLog {
    fn already_sent(&self, key: &str) -> bool;
    fn mark_sent(&mut self, key: String);
}

/// The simplest log: a set in memory. The real job stores it in Postgres.
impl SentLog for BTreeSet<String> {
    fn already_sent(&self, key: &str) -> bool {
        self.contains(key)
    }

    fn mark_sent(&mut self, key: String) {
        self.insert(key);
    }
}
