mod common;

use common::d;
use jobs_alerts::model::{Channel, Contact, ExpiringItem};
use jobs_alerts::t00_plan_alerts::plan_alerts;

fn item(staff_number: u32, code: &str, valid_until: chrono::NaiveDate) -> ExpiringItem {
    ExpiringItem {
        staff_number,
        code: code.to_string(),
        valid_until,
    }
}

fn contacts() -> Vec<Contact> {
    vec![
        Contact {
            staff_number: 48213,
            email: Some("amina@example.test".into()),
            phone: Some("+254700000001".into()),
            whatsapp_opt_in: false,
            push_token: Some("push-48213".into()),
        },
        Contact {
            staff_number: 10007,
            email: None,
            phone: Some("+254700000002".into()),
            whatsapp_opt_in: true,
            push_token: None,
        },
    ]
}

fn today() -> chrono::NaiveDate {
    d(2026, 10, 16)
}

#[test]
fn reminds_on_the_reminder_days_only() {
    let items = [
        item(10007, "DG", d(2026, 12, 15)),    // 60 days
        item(10007, "FIRE", d(2026, 11, 15)),  // 30 days
        item(10007, "RAMP", d(2026, 11, 14)),  // 29 days: no reminder today
        item(10007, "AVSEC", d(2026, 10, 15)), // expired yesterday
        item(10007, "OLD", d(2026, 10, 14)),   // expired 2 days ago: no more reminders
    ];
    let codes: Vec<(String, i64)> = plan_alerts(today(), &items, &contacts())
        .into_iter()
        .map(|a| (a.code, a.days_left))
        .collect();
    assert_eq!(
        codes,
        [
            ("AVSEC".to_string(), -1),
            ("DG".to_string(), 60),
            ("FIRE".to_string(), 30)
        ]
    );
}

#[test]
fn one_alert_per_channel_with_the_right_address() {
    let items = [item(48213, "DG", d(2026, 10, 23))]; // 7 days
    let alerts = plan_alerts(today(), &items, &contacts());
    let sent: Vec<(Channel, &str)> = alerts.iter().map(|a| (a.channel, a.to.as_str())).collect();
    assert_eq!(
        sent,
        [
            (Channel::Email, "amina@example.test"),
            (Channel::Sms, "+254700000001"),
            (Channel::Push, "push-48213"),
        ]
    );
    assert!(
        alerts
            .iter()
            .all(|a| a.message == "Your DG expires in 7 days.")
    );
}

#[test]
fn whatsapp_instead_of_sms_when_they_opted_in() {
    let items = [item(10007, "DG", d(2026, 10, 16))]; // today
    let alerts = plan_alerts(today(), &items, &contacts());
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].channel, Channel::WhatsApp);
    assert_eq!(alerts[0].message, "Your DG expires today.");
}

#[test]
fn nobody_to_tell_means_no_alert() {
    let items = [item(99999, "DG", d(2026, 10, 17))];
    assert!(plan_alerts(today(), &items, &contacts()).is_empty());
}

#[test]
fn sorted_by_person_then_code_then_channel() {
    let items = [
        item(48213, "FIRE", d(2026, 10, 17)),
        item(10007, "DG", d(2026, 10, 17)),
        item(48213, "DG", d(2026, 10, 17)),
    ];
    let order: Vec<(u32, String, Channel)> = plan_alerts(today(), &items, &contacts())
        .into_iter()
        .map(|a| (a.staff_number, a.code, a.channel))
        .collect();
    assert_eq!(order[0], (10007, "DG".to_string(), Channel::WhatsApp));
    assert_eq!(order[1], (48213, "DG".to_string(), Channel::Email));
    assert_eq!(order[4], (48213, "FIRE".to_string(), Channel::Email));
    assert_eq!(order.len(), 7);
}
