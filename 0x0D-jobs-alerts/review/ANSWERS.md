# Answers

1. **"Every hour on both servers" with no claim on the work** means each reminder can be sent up to 48 times a day. The PR description presents that as a feature, so explain the cost to the person and to the SMS bill. Run the job once a day at a fixed local time, and make it safe on several servers: a sent-log keyed by alert, plus `FOR UPDATE SKIP LOCKED` (or a Postgres advisory lock) so only one server works a batch.

2. **It marks the alert sent *before* sending.** If the send fails, or the server restarts mid-loop, the person is recorded as told and never hears anything. That's the "one person got none" bug. Record after a successful send (task 1).

3. **It retries forever, every 100 ms, on every error.** An invalid phone number is a permanent error: this loop never ends, the job never finishes, and the provider sees ten requests a second. Retry only transient errors, a limited number of times, with growing waits (task 2), and report the permanent ones.

4. **`std::thread::sleep` in async code** blocks the runtime thread for every retry. Use `tokio::time::sleep`.

5. **Only the 30-day reminder exists.** Someone who first appears at 20 days (a new hire, a data fix) gets nothing until it's too late. The agreed schedule (60, 30, 7, 1, 0, −1 days) should be planned from data (task 0).

6. **`Utc::now()` for "today"** is the wrong date for three hours every night in Nairobi, and makes the job impossible to test. Take `today` in Nairobi time, once, and pass it in.

7. **The message is too long, and it has an emoji.** The 👋 switches the whole SMS to the 70-character encoding, so this message costs three SMS parts instead of one. Times 800 staff, that's real money. Keep it short and plain, or fit it (task 100).

8. **SMS only**, even for staff who've chosen WhatsApp or the app. And no check that `row.phone` exists.

9. **`unwrap()` on the database calls**: one bad query kills the whole run silently, in a background job nobody's watching. Return errors, log them with context, and alert a person when the job fails.

10. **`println!` for logging** goes nowhere useful on a server. Use `tracing` with structured fields (staff number, code, attempt), and *never* log the full message or phone number. They're personal data.

Lead with 1–3. Each of those is a production incident waiting to happen.
