# 0x0C. Async Rust and the backend

**Week 7** · The TWA server: async Rust, an Axum web service, errors as HTTP responses, and authentication.

## What you'll be able to do

- Explain what `async` and `.await` do, and avoid the two mistakes that freeze an async server
- Build HTTP endpoints with Axum: routes, path and query parameters, JSON in and out, shared state
- Turn every error into the right HTTP status and a JSON body the app can use
- Protect routes with a bearer token, and test the whole service without opening a network port

## Read first

- The Book: **Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams**
- The Tokio tutorial: **Hello Tokio**, **Spawning**, **Shared state**
- Axum docs: the crate overview, **Extractors**, **Error handling**, **Middleware** (`from_fn`)

## Async in plain words

A web server spends most of its time *waiting*: for the database, for an SMS provider, for a slow phone on a station's Wi-Fi. `async` lets one thread juggle thousands of waiting requests. Each `.await` says "I'm waiting here; run something else meanwhile".

The two mistakes that freeze an async server:

1. **Blocking inside async code**: `std::thread::sleep`, a heavy calculation, or a blocking file or database call. It stops every other request sharing that thread. Use the async version (`tokio::time::sleep`), or move the work off with `tokio::task::spawn_blocking`.
2. **Holding a lock across an `.await`**: a `std::sync::Mutex` guard kept while waiting blocks every other request that needs the lock, and can deadlock. Take what you need, drop the guard, *then* await.

## How this project is laid out

- `src/model.rs` (given): `Employee`, `NewEmployee`, and `AppState`, the state every handler shares.
- `src/app.rs` (given): wires each route to its handler. `routes(state)` is the app without authentication, and `app(state)` is the same plus the auth check from task 4.
- Tasks 1–4 and 100 are the pieces `app.rs` connects. **Do task 1 first**: every error response goes through it.

The tests send requests straight to the router with `tower::ServiceExt::oneshot`, so no port is opened and nothing touches the network.

## Tasks

```bash
cargo test -p async-backend
```

### 0. Async basics (`t00_async_basics.rs`)

Two helpers the server will need. `with_timeout` wraps any future in `tokio::time::timeout`, because a call to an outside service must never hang forever. `count_all` runs one lookup per station *at the same time* with a `tokio::task::JoinSet`, then returns the results sorted. Tasks finish in any order, and the output must not depend on that.

The tests use `#[tokio::test(start_paused = true)]`: Tokio's clock is frozen and skips ahead instantly, so a test of a 30-second timeout takes no time at all.

### 1. Errors as HTTP responses (`t01_api_error.rs`)

Implement `IntoResponse` for `ApiError`. Each variant gets a status (`404`, `409`, `422`, `401`, `500`) and a JSON body like `{"error": "not_found", "message": "No employee 48213"}`. For `Internal`, log the detail but send the client only a generic message: internal details in responses help attackers, not users.

### 2. Employee endpoints (`t02_employees.rs`)

- `GET /employees/{staff_number}`: `Path` extracts the number, then look it up.
- `POST /employees`: `Json<NewEmployee>` parses the body, then validate, refuse duplicates, store, and return **201 Created**.

The state uses a `std::sync::RwLock`. That's fine *because no handler awaits while holding it*. Read or write, then let go.

### 3. The transfer check (`t03_transfers.rs`)

`POST /transfers/check` takes a staff number, department and role, and answers whether they're ready and which codes are missing. This is a simplified gap check that only looks at codes held. In the real server, this handler calls your 0x09 engine with the employee's holdings from the 0x0B database.

### 4. Authentication (`t04_auth.rs`)

`is_authorized` checks for `Authorization: Bearer <token>`. `app.rs` runs it in a middleware for every route except `/health`. In TWA, the token will be a Supabase login token, checked against Supabase's signing key, but the shape of the middleware is the same.

Compare tokens without stopping at the first different character (`constant_time_eq` in the solution), or an attacker can learn the token one character at a time by timing your responses.

### 100. A filtered list (`t100_list.rs`) — advanced

`GET /employees?station=NBO&limit=2&after=10007` uses `Query<ListParams>` for optional parameters: filter by station, return at most `limit` (default 50, never more than 200), starting after a staff number. Paging by "after this key" stays correct even when rows are added between pages, which paging by page number doesn't.

## Review it

Open [`review/README.md`](review/README.md): a handler that blocks the runtime, holds a lock across an await, and leaks internal errors.

## Explain it

The client asks: *"What happens if 300 staff at JKIA all open the app at 6 a.m. when shifts change?"* Write four sentences on how the server handles many requests at once, and the one thing in the code that would make it fall over.

## Checkpoint

- `cargo test -p async-backend` passes and clippy is clean
- Run the server for real: add a `main.rs` that starts it with `tokio::net::TcpListener` and `axum::serve`, then call `/health` from your browser and `/employees/1` from `curl` with and without the token

## When you're stuck

- **"future cannot be sent between threads safely"**: something that isn't `Send` (often a `std::sync::MutexGuard`) is alive across an `.await`. Drop it before awaiting.
- **A handler won't compile with a confusing trait error**: the *last* extractor must be the one that reads the body (`Json`). Put `State` and `Path` first.
- **"422 Unprocessable Entity" with a plain-text body**: Axum rejected the JSON before your handler ran (a missing or misspelled field). Check the field names.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
