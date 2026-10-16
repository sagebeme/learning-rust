//! Given: what the 06:00 job would send on a given morning, for the course's sample staff.
//!
//! ```bash
//! cargo run -p jobs-alerts --example morning_run            # the morning of 25 October 2026
//! cargo run -p jobs-alerts --example morning_run 2026-10-31 # any other morning
//! ```
//!
//! It uses your task code (tasks 0, 1, 4 and 100), and a fake sender that prints instead of
//! sending, so nothing ever leaves your machine. Run it twice in one go and the second run sends
//! nothing: that's task 1 working.

use std::collections::BTreeSet;

use chrono::NaiveDate;
use jobs_alerts::model::{Alert, AlertSender, Contact, SendError};
use jobs_alerts::t00_plan_alerts::plan_alerts;
use jobs_alerts::t01_daily_run::run_daily;
use jobs_alerts::t04_from_engine::expiring_items;
use jobs_alerts::t100_fit_sms::fit_sms;

struct PrintingSender;

impl AlertSender for PrintingSender {
    async fn send(&self, alert: &Alert) -> Result<(), SendError> {
        println!(
            "  {:?} to staff {} ({}): {}",
            alert.channel,
            alert.staff_number,
            alert.to,
            fit_sms(&alert.message)
        );
        Ok(())
    }
}

fn contacts() -> Vec<Contact> {
    let contact = |staff_number, phone: &str, whatsapp_opt_in, email: Option<&str>| Contact {
        staff_number,
        email: email.map(str::to_string),
        phone: Some(phone.to_string()),
        whatsapp_opt_in,
        push_token: None,
    };
    vec![
        contact(48213, "+254700000001", true, None),
        contact(10007, "+254700000002", false, Some("kip@example.test")),
        contact(20020, "+254700000003", false, None),
        contact(30303, "+254700000004", true, None),
    ]
}

#[tokio::main]
async fn main() {
    let today = std::env::args()
        .nth(1)
        .map(|arg| arg.parse::<NaiveDate>().expect("a date like 2026-10-25"))
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2026, 10, 25).expect("a real date"));

    let items = expiring_items(
        &twa_engine::sample::catalogue(),
        &twa_engine::sample::holdings(),
    );
    let alerts = plan_alerts(today, &items, &contacts());

    let mut sent_log = BTreeSet::new();
    println!("Morning of {today}: {} alert(s) planned", alerts.len());
    let first = run_daily(&alerts, &PrintingSender, &mut sent_log).await;
    println!("First run: {first:?}");
    let second = run_daily(&alerts, &PrintingSender, &mut sent_log).await;
    println!("Second run, same morning: {second:?}");
}
