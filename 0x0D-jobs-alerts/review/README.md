# Review: "Daily expiry alerts"

A teammate opens this PR: *"Cron job for expiry reminders. Runs every hour on both servers so we never miss one. Retries until it works."* Review it.

```rust
pub async fn hourly_job(db: &PgPool, sms: &SmsClient) {
    let today = Utc::now().date_naive();
    let rows = expiring_in_days(db, today, 30).await.unwrap();
    for row in rows {
        db_mark_alert_sent(db, row.staff_number, &row.code).await.unwrap();
        let text = format!(
            "Habari {}! 👋 Your {} certificate expires on {}. Please contact the training office at the NBO terminal building as soon as possible to book.",
            row.name, row.code, row.valid_until
        );
        loop {
            match sms.send(&row.phone, &text).await {
                Ok(_) => break,
                Err(e) => {
                    println!("retrying: {e}");
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
}
```
