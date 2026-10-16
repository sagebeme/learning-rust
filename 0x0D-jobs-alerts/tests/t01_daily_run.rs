mod common;

use std::collections::BTreeSet;

use common::{FakeSender, alert};
use jobs_alerts::model::{Channel, SendError};
use jobs_alerts::t01_daily_run::{RunSummary, dedupe_key, run_daily};

#[test]
fn the_key_names_the_person_qualification_expiry_reminder_and_channel() {
    assert_eq!(
        dedupe_key(&alert(48213, "DG", Channel::Sms)),
        "48213:DG:2026-11-15:30:Sms"
    );
    assert_ne!(
        dedupe_key(&alert(48213, "DG", Channel::Sms)),
        dedupe_key(&alert(48213, "DG", Channel::Email))
    );
}

#[tokio::test]
async fn sends_everything_the_first_time() {
    let alerts = [alert(1, "DG", Channel::Sms), alert(2, "DG", Channel::Email)];
    let sender = FakeSender::default();
    let mut log = BTreeSet::new();
    let summary = run_daily(&alerts, &sender, &mut log).await;
    assert_eq!(
        summary,
        RunSummary {
            sent: 2,
            skipped: 0,
            failed: 0
        }
    );
    assert_eq!(sender.attempts(), 2);
}

#[tokio::test]
async fn running_twice_never_sends_twice() {
    let alerts = [alert(1, "DG", Channel::Sms), alert(2, "DG", Channel::Email)];
    let sender = FakeSender::default();
    let mut log = BTreeSet::new();
    run_daily(&alerts, &sender, &mut log).await;
    let second = run_daily(&alerts, &sender, &mut log).await;
    assert_eq!(
        second,
        RunSummary {
            sent: 0,
            skipped: 2,
            failed: 0
        }
    );
    assert_eq!(sender.attempts(), 2);
}

#[tokio::test]
async fn a_failed_alert_is_tried_again_next_run() {
    let alerts = [alert(1, "DG", Channel::Sms), alert(2, "DG", Channel::Email)];
    let sender = FakeSender::answering(vec![Err(SendError::Transient("timeout".into())), Ok(())]);
    let mut log = BTreeSet::new();

    let first = run_daily(&alerts, &sender, &mut log).await;
    assert_eq!(
        first,
        RunSummary {
            sent: 1,
            skipped: 0,
            failed: 1
        }
    );

    let second = run_daily(&alerts, &sender, &mut log).await;
    assert_eq!(
        second,
        RunSummary {
            sent: 1,
            skipped: 1,
            failed: 0
        }
    );
    let tried: Vec<u32> = sender
        .sent
        .lock()
        .unwrap()
        .iter()
        .map(|a| a.staff_number)
        .collect();
    assert_eq!(tried, [1, 2, 1]);
}
