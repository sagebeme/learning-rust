//! Task 0: decide who gets which alert today.
//!
//! For each item whose days left (from `today` to `valid_until`) is one of `REMINDER_DAYS`, look
//! up the person's contact (no contact: no alerts) and make one alert per channel they can be
//! reached on:
//! - `Email` if they have an email address
//! - for a phone number: `WhatsApp` if they've opted in, otherwise `Sms` (never both)
//! - `Push` if they have a push token
//!
//! `to` is the address, number or token; `message` comes from `message_for`. Return the alerts
//! sorted by staff number, then code, then channel.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use chrono::NaiveDate;

use crate::model::{Alert, Channel, Contact, ExpiringItem, REMINDER_DAYS, message_for};

pub fn plan_alerts(today: NaiveDate, items: &[ExpiringItem], contacts: &[Contact]) -> Vec<Alert> {
    todo!()
}
