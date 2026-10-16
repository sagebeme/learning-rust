# Answers

1. **A `std::sync::Mutex` guard held across `.await`.** The cache is locked before the database query and only released at the end, across two awaits and seconds of work. Every other request for this report waits behind it. With a `std` mutex, this often doesn't even compile in a spawned task ("future cannot be sent between threads safely"); where it does, it serialises the endpoint. Check the cache, **drop the guard**, do the slow work, then lock briefly again to store the result.

2. **`std::thread::sleep` in async code** blocks the whole worker thread: every other request scheduled on it stops for half a second. There's no reason to sleep at all. Delete it. (If a delay were ever needed, `tokio::time::sleep(...).await`.)

3. **Two seconds of CPU work inside a handler** (`build_pdf_summary`) blocks a runtime thread too. Run it with `tokio::task::spawn_blocking`, or better, move PDF building and emailing to the background job (0x0D) and have this endpoint just return data.

4. **Sending email from a GET request.** Fetching a report should not have side effects: refreshing the dashboard emails every manager again. And `.ok()` hides send failures. Move it to a job, with run-once guarantees.

5. **Panics on bad input.** `q.get("days").unwrap().parse().unwrap()` crashes the request for a missing or non-numeric `days`. Use a typed `Query<ReportParams>` with `days: u32` (Axum then returns 400 by itself) and a sensible maximum.

6. **Leaking internals.** `format!("query failed: {e:?}")` sends database error details (table names, SQL fragments) to the browser. Return `ApiError::Internal`: log the detail on the server and send a generic message (task 1). `Result<_, String>` also always becomes a 500, even for bad input.

7. **`now()` in SQL** is the database server's clock and time zone, not "today in Nairobi". Pass the date in from the app layer, as the engine does.

8. **The cache never expires**, so after the first request the report is stale forever. If a cache is needed at all (measure first), give entries a time limit.

The pattern: the PR description says "takes a few seconds but works". Under load, 1–3 make it take a few seconds *for everyone, on every endpoint*. Point that out. It turns "slow report" into "server outage at shift change", which is the right priority.
