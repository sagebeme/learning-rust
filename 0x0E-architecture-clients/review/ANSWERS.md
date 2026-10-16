# Answers

## PR 1 — refresher training

- **Blocker: the rule is invented in code.** "A refresher adds 365 days" is a business rule, and probably the wrong one. Many regimes reset validity *from the refresher date*, or require the refresher within a window before expiry. Ask the client for the rule, then model it on the qualification type (`refresher_interval_months`, an allowed window) instead of a constant in `holding_state`.
- **Blocker: it ignores *when* the refresher happened.** A refresher taken three years ago still adds a year.
- **Should fix: days, not calendar months**, unlike every other validity rule in the engine.
- **Should fix: no tests, and no fixtures cases.** A change to the engine without new cases in the shared fixtures file means the app and server can't be shown to agree.

## PR 2 — a cache table

- **Blocker: a nightly copy is wrong for up to 24 hours.** Someone renewed this morning still shows as expired on the staff page until tomorrow, and a supervisor may pull them off shift. Find out *why* the page is slow first (an `EXPLAIN ANALYZE`, a missing index on `holdings.staff_number`, an N+1 query in the server). Then fix that.
- **Should fix: `CREATE TABLE AS` with `e.*`** silently changes shape when `employees` gains a column, and the codes as a comma-separated string throw away structure the app then has to re-parse.
- **Should fix: it's a schema change without a migration file.** It must go through the migrations folder, or another environment won't have it.

## PR 3 — bulk import

- **Blocker: SQL injection.** `format!` into SQL, with values straight from a CSV file. A name like `O'Brien` breaks it, and a crafted cell can do far worse. Use `$1` placeholders and `.bind`.
- **Blocker: errors are thrown away, and the count lies.** `.ok()` swallows every failure, yet `imported` counts every line, so HR is told 900 were imported when 400 were. Use one transaction, all-or-nothing, and report the failing row (0x0B task 100).
- **Blocker: `fields[3]` panics** on a short line, and the header line is imported as an employee.
- **Should fix: no authentication check is visible.** Bulk-changing staff data needs an admin role, not just any logged-in user.
- **Should fix: "Tested with the HR export"** means real staff data went through a test. Use fake data.

## PR 4 — home screen status

- **Blocker: a second, different validity rule.** `completed_on + 730 days` is not the engine's rule: it ignores calendar months, grace, recency and each qualification's own validity. The screen and the server will disagree. Call the engine (0x09) through the app core.
- **Should fix: `Local::now()`** uses the device's clock and time zone. Pass "today" in from one place, computed in Nairobi time.
- **Should fix: `unwrap()` on the lock** crashes the app (0x0A task 2), and `Vec<(String, bool)>` loses everything the screen should show: *expiring in 5 days* and *in grace* are very different messages from a bare `true`.

## PR 5 — supervisor copies

- **Blocker: personal data to the wrong people, without a decision.** Copying every reminder to supervisors shares one person's training status with someone else, by SMS. That may be fine, but it's a data-protection question for the client, not a code change. Get the decision in writing (and in a decision record) first.
- **Blocker: duplicates.** Supervisor copies bypass the sent-log, so every run re-sends them, and a supervisor with 30 staff gets 30 texts a day.
- **Should fix: errors swallowed with `.ok()`, and copies sent as SMS regardless of the supervisor's channel.** Plan supervisor alerts with the same planner (a digest per supervisor per day is kinder and cheaper), through the same idempotent run.

## How the answers were ordered

Every PR has at least one blocker, and none of them is about style. In a real week of reviews, most of your value is in the first two bullets of each list. Spend your time there, and mark style points as nits or leave them for a formatter.
