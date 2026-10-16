# 0x0D. Jobs and alerts

**Week 7** · The daily expiry check: deciding who to tell, sending by email, SMS, WhatsApp and push, and never sending the same alert twice.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x0C done | `cargo test -p async-backend` passes | Go back to [0x0C](../0x0C-async-backend) |
| To know what UTC and time zones are | You can say what time it is in UTC at 06:00 in Nairobi | The [UTC article](https://en.wikipedia.org/wiki/Coordinated_Universal_Time) intro; Nairobi is UTC+3 all year |
| Provider accounts | Not needed: this project only uses a fake sender | When you build the real job, you'll need sandbox accounts with [Africa's Talking](https://developers.africastalking.com/docs/sms/overview), the [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api) and [Firebase](https://firebase.google.com/docs/cloud-messaging) |

## The story so far

The server tells HR that Amina isn't ready yet, partly because her dangerous goods certificate expires on 1 November. But nobody should have to ask: TWA must *tell* her, in time, on the channel she prefers, exactly once. That's the 06:00 job, and it starts from the engine's dates.

This project uses the engine too (task 4). See "Using your own engine" in [0x0A's README](../0x0A-tauri-core/README.md#using-your-own-engine) to switch it to yours.

## What you'll be able to do

- Plan alerts as plain data first, and send them as a separate step
- Make a job safe to run twice (and safe to crash halfway through)
- Retry the failures that are worth retrying, with backoff, and give up on the rest
- Schedule a daily job in Nairobi time, correctly

## Read first

- Tokio docs: [`tokio::time`](https://docs.rs/tokio/latest/tokio/time/index.html) (`sleep`, `timeout`, and [pausing time in tests](https://docs.rs/tokio/latest/tokio/time/fn.pause.html))
- The Rust blog post [announcing async fn in traits](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html), which explains why public traits often write `-> impl Future<Output = ...> + Send` instead
- Skim the provider docs you'll use in TWA: [Africa's Talking SMS](https://developers.africastalking.com/docs/sms/overview), the [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api) (message templates and opt-in rules) and [Firebase Cloud Messaging](https://firebase.google.com/docs/cloud-messaging). Don't call any of them in this project: the course uses a fake sender.

**Go deeper:** AWS's article on [timeouts, retries and backoff with jitter](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/), and Postgres's [`SKIP LOCKED`](https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE) for running a job on several servers.

## The shape of a good job

```
find what's expiring (engine)  →  plan the alerts (pure)  →  send them (slow, can fail)  →  record what was sent
```

Planning is pure: same input, same alerts, easy to test. Sending is where the outside world gets involved, so it sits behind a trait (`AlertSender`), and tests use a fake. Recording what was sent is what makes the job **idempotent**: run it again, after a crash or by accident, and nobody gets a second SMS at 6 a.m.

## Tasks

```bash
cargo test -p jobs-alerts
```

`src/model.rs` is given: `Contact`, `ExpiringItem`, `Alert`, `SendError`, the `AlertSender` and `SentLog` traits, and `message_for`.

### 0. Plan the alerts (`t00_plan_alerts.rs`)

Reminders go out 60, 30, 7 and 1 days before, on the day, and the day after expiry. Each person gets one alert per channel they can be reached on. WhatsApp replaces SMS for people who've opted in: WhatsApp business messages need the person's consent, and SMS costs per message. Sort the result, so a log of what was planned reads the same every time.

### 1. Never twice (`t01_daily_run.rs`)

The key identifies "the same alert" across runs: person, qualification, expiry date, which reminder, channel. Run the list through the sender, skipping keys already in the log, and **only record a key after the send succeeded**. If you record first and the send then fails, the person is never told. If you send and never record, they're told every run. The tests check both.

In the real job, the log is a Postgres table with the key as its primary key. When the job runs on more than one server, a jobs table claimed with `SELECT ... FOR UPDATE SKIP LOCKED` makes sure only one of them sends each batch.

### 2. Retry with backoff (`t02_retry.rs`)

`Transient` errors (timeouts, a busy provider) are worth retrying, after a growing wait: 1s, 2s, 4s… so a struggling provider isn't hammered. `Permanent` errors (an invalid number) never get better: stop immediately, and log it for a person to fix. `match` with a binding, `Err(error @ SendError::Permanent(_))`, keeps the error so you can return it.

The tests run with Tokio's clock paused, so "wait 7 seconds" takes no time, and they check exactly how long the code *would* have waited.

### 3. Run at 06:00 Nairobi time (`t03_schedule.rs`)

A server usually runs in UTC. Nairobi is UTC+3 all year (no daylight saving), so a `FixedOffset` is the right tool. Work out "today at 06:00" in Nairobi; if that's already passed, it's tomorrow. Then convert back to UTC. The tests include the trap: at 22:30 UTC it's already tomorrow in Nairobi.

### 4. From the engine to the job (`t04_from_engine.rs`)

The job's input comes from the engine: for every person and holding, the engine's `valid_until`. A renewal must win over the old certificate, or the job keeps reminding Amina about a certificate she's already replaced. The last test runs the whole chain, engine → items → planned alerts, for the morning of 25 October 2026: Amina gets one WhatsApp, a week before her dangerous goods certificate expires.

Then run it yourself:

```bash
cargo run -p jobs-alerts --example morning_run
cargo run -p jobs-alerts --example morning_run 2026-11-02
```

The example (`examples/morning_run.rs`, given) plans and "sends" a morning's alerts for the story's staff, printing them instead of sending, then runs again to show nothing goes out twice.

### 100. Fit one SMS (`t100_fit_sms.rs`) — advanced

From the TypeScript course's caption fitter: one SMS is 160 characters of plain text but only 70 once a single non-basic character appears (an accent, an emoji), because the whole message switches encoding. Cut long messages at a word boundary and mark the cut. A two-part SMS costs twice as much, and 800 staff × 6 reminders adds up.

## Review it

Open [`review/README.md`](review/README.md): a daily job that sends duplicates, spins on retries and runs at the wrong time.

## Explain it

The training manager asks: *"Last month two people said they got the same reminder three times, and one said they got none. What happened, and how do we know it won't happen again?"* Write the explanation you'd give, using what this project taught you, in five sentences or fewer.

## Checkpoint

- `cargo test -p jobs-alerts` passes and clippy is clean
- You can explain, without notes, why the log is written *after* sending and not before

## When you're stuck

- **"use of `async fn` in public traits is discouraged"**: that's why `AlertSender` is written with `-> impl Future<...> + Send`. Implementations can still use `async fn`.
- **A retry test takes ages**: you used `std::thread::sleep`. Use `tokio::time::sleep`.
- **Off by three hours**: you mixed UTC and Nairobi dates. Convert `now` to Nairobi *first*, then take the date.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
