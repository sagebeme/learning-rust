# Glossary

Words the course uses, in plain language. The project where each one is taught properly is in brackets.

## TWA words

| Word | Meaning |
| --- | --- |
| **Qualification** | A licence, certificate or course someone must hold to do a job: airside safety, dangerous goods, a forklift licence |
| **Holding** | One person's record of one qualification: what it is, and when they completed it (0x09) |
| **Validity** | How long a qualification lasts after it's completed, for example 24 calendar months (0x08) |
| **Grace period** | Days after expiry during which the person may still work, while they renew (0x04) |
| **Warning window** | Days before expiry when the system starts reminding people (0x04) |
| **Recency** | A rule that the qualification must have been *used* recently, not just held, to stay current (0x09) |
| **Lapsed** | Still within its validity, but not used recently enough (0x09) |
| **Refresher** | A shorter course that renews a qualification |
| **Prerequisite** | A qualification you must hold before another one counts. Forklift needs airside safety (0x07, 0x09) |
| **Equivalent** | A different qualification accepted instead, such as an external ICAO certificate for dangerous goods (0x09) |
| **Requirements matrix** | What each department and role requires (0x09) |
| **Gap check** | For one person and one role: what's met, what's expiring, what's missing (0x09) |
| **Station** | One of the company's four airports: NBO (Nairobi), MBA (Mombasa), KIS (Kisumu), EDL (Eldoret) |

## Rust words

| Word | Meaning |
| --- | --- |
| **Crate** | A Rust library or program. Each project folder in this course is one crate |
| **Cargo** | Rust's build tool: it compiles, tests and downloads libraries (0x00) |
| **Workspace** | Several crates built together, like this repo (0x00, 0x0E) |
| **Ownership** | Every value has exactly one owner, and it's freed when the owner goes away (0x03) |
| **Move** | Giving a value to a new owner; the old variable can't use it afterwards (0x03) |
| **Borrow** | Using a value without owning it: `&` to read, `&mut` to change (0x03) |
| **Slice** | A borrowed view into part of a list or text, like `&str` or `&[u32]` (0x03) |
| **Lifetime** | How long a borrow is valid; written `'a` when the compiler needs to be told (0x07) |
| **Stack and heap** | Two places memory lives: the stack is fast and fixed-size; the heap is for data whose size isn't known in advance (0x00, 0x03) |
| **`struct` / `enum`** | A type that's "this *and* that" / "this *or* that" (0x04) |
| **`Option`** | A value that might not be there: `Some(value)` or `None` (0x04) |
| **`Result`** | An operation that might fail: `Ok(value)` or `Err(error)` (0x05) |
| **`?`** | "If this failed, return the error now" (0x05) |
| **Panic** | The program stops with an error because something that should be impossible happened (0x05) |
| **Trait** | A set of methods a type promises to have, like an interface (0x08) |
| **Generic** | Code written once for many types: `fn f<T: Trait>(...)` (0x08) |
| **Trait object** | `dyn Trait`: different types in one list, chosen at run time (0x08) |
| **`Box`, `Rc`, `Arc`** | Pointers that own heap data: one owner; several owners on one thread; several owners across threads (0x07) |
| **`Mutex`** | A lock: only one thread may change the data at a time (0x07, 0x0A) |
| **Iterator** | Something that yields items one at a time; `map`, `filter` and `collect` work on it (0x06) |
| **Closure** | A small unnamed function, like `\|x\| x + 1` (0x06) |
| **Macro** | Code that writes code, recognisable by the `!`: `println!`, `vec!`, `format!` |
| **`async` / `.await`** | Code that waits without blocking the thread, so one thread can juggle many requests (0x0C) |
| **Future** | A value that will be ready later; `.await` waits for it (0x0C) |
| **Runtime (Tokio)** | The engine that runs async code (0x0C) |
| **Clippy** | Rust's linter: it flags code that works but could be better (0x00) |
| **`todo!()`** | A placeholder that compiles but panics when run. Every task starts with one |

## Working words

| Word | Meaning |
| --- | --- |
| **Pull request (PR)** | A proposed change, reviewed before it joins the main code (0x03 onwards) |
| **CI** | Continuous integration: the robot that runs every check on every change |
| **Fixture** | Saved test data, such as the transfer cases in `0x09-rules-engine/tests/fixtures/` (0x09) |
| **Property test** | A test that checks a rule against hundreds of random inputs (0x08) |
| **Idempotent** | Safe to do twice: running the alert job again sends nothing new (0x0D) |
| **Transaction** | Database changes that all happen, or none do (0x0B) |
| **Migration** | A versioned change to the database's structure (0x0B) |
| **Decision record (ADR)** | A one-page note of a decision, why it was made, and what it costs (week 1, 0x0E) |
