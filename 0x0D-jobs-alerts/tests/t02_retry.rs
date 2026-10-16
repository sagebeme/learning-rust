mod common;

use std::time::Duration;

use common::{FakeSender, alert};
use jobs_alerts::model::{Channel, SendError};
use jobs_alerts::t02_retry::{RetryPolicy, send_with_retry};
use tokio::time::Instant;

const POLICY: RetryPolicy = RetryPolicy {
    max_attempts: 4,
    first_delay: Duration::from_secs(1),
};

fn transient() -> Result<(), SendError> {
    Err(SendError::Transient("provider busy".into()))
}

// The clock is paused, so waiting 1 + 2 + 4 seconds takes no real time, and the elapsed time
// tells us exactly how long the code would have waited.

#[tokio::test(start_paused = true)]
async fn succeeds_first_time_without_waiting() {
    let sender = FakeSender::default();
    let started = Instant::now();
    assert_eq!(
        send_with_retry(&sender, &alert(1, "DG", Channel::Sms), POLICY).await,
        Ok(1)
    );
    assert_eq!(started.elapsed(), Duration::ZERO);
}

#[tokio::test(start_paused = true)]
async fn backs_off_1_2_4_seconds() {
    let sender = FakeSender::answering(vec![transient(), transient(), transient(), Ok(())]);
    let started = Instant::now();
    assert_eq!(
        send_with_retry(&sender, &alert(1, "DG", Channel::Sms), POLICY).await,
        Ok(4)
    );
    assert_eq!(started.elapsed(), Duration::from_secs(1 + 2 + 4));
}

#[tokio::test(start_paused = true)]
async fn gives_up_after_the_last_attempt() {
    let sender = FakeSender::answering(vec![transient(); 10]);
    let result = send_with_retry(&sender, &alert(1, "DG", Channel::Sms), POLICY).await;
    assert_eq!(result, transient().map(|_| 0));
    assert_eq!(sender.attempts(), 4);
}

#[tokio::test(start_paused = true)]
async fn a_permanent_failure_is_never_retried() {
    let sender = FakeSender::answering(vec![Err(SendError::Permanent("invalid number".into()))]);
    let started = Instant::now();
    let result = send_with_retry(&sender, &alert(1, "DG", Channel::Sms), POLICY).await;
    assert_eq!(result, Err(SendError::Permanent("invalid number".into())));
    assert_eq!(sender.attempts(), 1);
    assert_eq!(started.elapsed(), Duration::ZERO);
}

#[tokio::test(start_paused = true)]
async fn zero_attempts_still_tries_once() {
    let sender = FakeSender::answering(vec![transient()]);
    let policy = RetryPolicy {
        max_attempts: 0,
        first_delay: Duration::from_secs(1),
    };
    assert!(
        send_with_retry(&sender, &alert(1, "DG", Channel::Sms), policy)
            .await
            .is_err()
    );
    assert_eq!(sender.attempts(), 1);
}
