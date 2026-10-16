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

use chrono::NaiveDate;

use crate::model::{Alert, Channel, Contact, ExpiringItem, REMINDER_DAYS, message_for};

fn channels(contact: &Contact) -> Vec<(Channel, String)> {
    let mut channels = Vec::new();
    if let Some(email) = &contact.email {
        channels.push((Channel::Email, email.clone()));
    }
    if let Some(phone) = &contact.phone {
        let channel = if contact.whatsapp_opt_in {
            Channel::WhatsApp
        } else {
            Channel::Sms
        };
        channels.push((channel, phone.clone()));
    }
    if let Some(token) = &contact.push_token {
        channels.push((Channel::Push, token.clone()));
    }
    channels
}

pub fn plan_alerts(today: NaiveDate, items: &[ExpiringItem], contacts: &[Contact]) -> Vec<Alert> {
    let mut alerts = Vec::new();
    for item in items {
        let days_left = (item.valid_until - today).num_days();
        if !REMINDER_DAYS.contains(&days_left) {
            continue;
        }
        let Some(contact) = contacts
            .iter()
            .find(|c| c.staff_number == item.staff_number)
        else {
            continue;
        };
        for (channel, to) in channels(contact) {
            alerts.push(Alert {
                staff_number: item.staff_number,
                code: item.code.clone(),
                valid_until: item.valid_until,
                days_left,
                channel,
                to,
                message: message_for(&item.code, days_left),
            });
        }
    }
    alerts.sort_by(|a, b| {
        (a.staff_number, &a.code, a.channel).cmp(&(b.staff_number, &b.code, b.channel))
    });
    alerts
}
