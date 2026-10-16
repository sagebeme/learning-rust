# 0x0B. Databases

**Week 6** · Storing TWA's data in Postgres from Rust: schema design, queries, migrations, and transactions that never leave a job half-done.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x0A done | `cargo test -p tauri-core` passes | Go back to [0x0A](../0x0A-tauri-core) |
| Docker, or Postgres installed | `docker run hello-world` prints a greeting, or `psql --version` prints a version | [Get Docker](https://docs.docker.com/get-started/get-docker/), or [install PostgreSQL](https://www.postgresql.org/download/) directly. The Setup section below has the one command to start it |
| Basic SQL: `SELECT`, `INSERT`, `WHERE`, `JOIN` | You can read `SELECT name FROM employees WHERE station_code = 'NBO'` | The [PostgreSQL tutorial](https://www.postgresql.org/docs/current/tutorial.html), chapter 2 (an hour) |
| To know what `.await` does, roughly | "Wait for the database to answer" | Enough for now: async is taught properly in [0x0C](../0x0C-async-backend) |

## The story so far

The app can show Amina's gap report. But the app's state disappears when it closes, and every station needs to see the same records. They belong in a database: Postgres, hosted on Supabase. This week Amina's records move into it, and come back out into the engine.

The database project uses the engine too (task 4). See "Using your own engine" in [0x0A's README](../0x0A-tauri-core/README.md#using-your-own-engine) to switch it to yours.

## What you'll be able to do

- Read and write a Postgres schema that enforces the rules itself (keys, foreign keys, checks)
- Query from Rust with `sqlx`, mapping rows into structs
- Turn database errors into errors your app understands ("that staff number already exists")
- Use transactions so related changes happen together or not at all

## Read first

- The [PostgreSQL tutorial](https://www.postgresql.org/docs/current/tutorial.html): chapters 2 and 3 (the SQL language, and advanced features: foreign keys, transactions)
- The [sqlx README](https://github.com/launchbadge/sqlx), and the [sqlx docs](https://docs.rs/sqlx) for `query`, `query_as`, `FromRow`, `Transaction` and `#[sqlx::test]`
- Supabase docs: [Database overview](https://supabase.com/docs/guides/database/overview). Supabase *is* Postgres, so everything here applies.

**Go deeper:** Postgres on [constraints](https://www.postgresql.org/docs/current/ddl-constraints.html), [`INSERT ... ON CONFLICT`](https://www.postgresql.org/docs/current/sql-insert.html#SQL-ON-CONFLICT) and [explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html) (`FOR UPDATE`); [Use The Index, Luke](https://use-the-index-luke.com/) for making queries fast.

## Setup: a Postgres to test against

The tests create a fresh database for every test, run the migrations, and throw it away afterwards (that's what `#[sqlx::test]` does). They need a Postgres server and a `DATABASE_URL`. The easiest way is Docker:

```bash
docker run --name twa-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:17
```

Then, in the same terminal you run the tests from:

```bash
# bash / macOS / Linux
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
# PowerShell
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres"
```

CI starts the same Postgres as a service, so the checks there are identical.

## The schema (`migrations/0001_init.sql`, given)

Read it first. Notice how much is enforced by the database itself:

- **Primary keys**: no two employees with the same staff number.
- **Foreign keys**: no employee at a station that doesn't exist, and no holding for an employee who doesn't exist.
- **Checks**: no blank names, staff numbers above zero, station codes of three capital letters.

Rules in the database hold however the data arrives: from your server, a migration script, or someone in the Supabase dashboard at 2 a.m. Rules only in your Rust code don't.

## Tasks

```bash
cargo test -p databases
```

Every function takes a `&PgPool` (a pool of database connections) and is `async`. You'll study async properly next week. For now, `.await` means "wait for the database to answer".

### 0. Add an employee (`t00_insert_employee.rs`)

`sqlx::query("INSERT ... VALUES ($1, $2, $3, $4)").bind(...).execute(pool).await`. Always use `$1` placeholders and `.bind`, **never** `format!` values into SQL: that's how SQL injection happens.

When the insert fails, look inside the error: `sqlx::Error::Database(e)` with `e.is_unique_violation()` means the staff number is taken, and `e.is_foreign_key_violation()` means the station doesn't exist. Map them to `DbError::DuplicateStaffNumber` and `DbError::UnknownStation`, so the server can return a clear message instead of a generic 500.

### 1. Record a qualification (`t01_record_holding.rs`)

An **upsert**: `INSERT ... ON CONFLICT (staff_number, code) DO UPDATE SET ...`. The twist: only replace the stored record if the new completion date is *later*. A delayed sync from an offline station must never overwrite a renewal with the older certificate. Add `WHERE holdings.completed_on < EXCLUDED.completed_on` to the update.

### 2. What's expiring (`t02_expiring_between.rs`)

A `JOIN` between holdings and employees, filtered by date range, in a fixed `ORDER BY`. Map rows into `ExpiringRow` with `sqlx::query_as::<_, ExpiringRow>(...)`. The struct derives `FromRow`, and its field names match the selected columns.

### 3. A transfer, all or nothing (`t03_transfer_employee.rs`)

A transfer changes the employee's department **and** writes an audit row. If the second step fails, the first must be undone. Otherwise the record says they moved, with no trace of when or why.

```rust
let mut tx = pool.begin().await?;
// ... queries using &mut *tx ...
tx.commit().await?;
```

If the function returns early with an error before `commit`, dropping `tx` rolls everything back. Use `SELECT ... FOR UPDATE` to read the current department, so two transfers of the same person can't interleave.

### 4. The database meets the engine (`t04_engine_bridge.rs`)

When the training office records that Amina completed a course, *who* decides when it expires? Not the database, and not the person typing: the engine. `record_completion` looks up the qualification type, asks the engine for `valid_until`, and stores both with task 1's upsert. `holdings_for` reads a person's records back as the engine's own `Holding` type. The last test stores Amina's records, reads them back, and checks that the engine gives exactly the same gap report from the database as from the sample data.

### 100. Import a roster (`t100_import_employees.rs`) — advanced

Insert many employees in one transaction: if any row fails, none are saved, and the error says which row. That's what HR expects from "import": all or nothing, not "the first 400 of 900 went in".

## Next steps

**Recency needs a column.** The `holdings` table has no "last used" date, so recency rules can't be checked from the database yet. Write a second migration (`migrations/0002_last_used.sql`) that adds `last_used_on DATE`, and load it in `holdings_for`.

## Checked queries

`sqlx::query!` (with a `!`) checks your SQL against the real database **at compile time**: misspell a column and it won't compile. It needs a database (or saved query data) while building, which is why this project uses the unchecked `query` functions. Try converting one query once your tests pass.

## Review it

Open [`review/README.md`](review/README.md): a repository module with SQL injection, a missing transaction, and an N+1 query.

## Explain it

The training manager asks: *"If the Mombasa office and the Nairobi office both update the same person at the same moment, whose change wins?"* Explain in three sentences what the database guarantees, and which decision is still up to the app.

## Checkpoint

- `cargo test -p databases` passes against a real Postgres, and clippy is clean
- You can point to the line in the schema that makes "an employee at an unknown station" impossible

## When you're stuck

- **"error communicating with database" / "connection refused"**: Postgres isn't running, or `DATABASE_URL` isn't set in *this* terminal.
- **"no rows returned by a query that expected to return at least one row"**: you used `fetch_one` where the row might not exist. Use `fetch_optional` and handle `None`.
- **"mismatched types… expected `i32`, found `i64`"**: Postgres `INTEGER` is `i32` in Rust, `BIGINT`/`BIGSERIAL` is `i64`.
- **A transaction "didn't work"**: check that every query inside it uses the transaction (`&mut *tx`), not the pool.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
