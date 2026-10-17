# Learning Rust

An 8-week, project-based Rust course for the developer building TWA: a training and qualification platform for an aviation ground-handling company in Nairobi. Every exercise uses TWA's world (staff, stations, licences, expiry dates) with fake data only, and the course is built around the job, not around Rust trivia:

| Your job on TWA | Where the course covers it |
| --- | --- |
| Architecture | Week 1 Tauri spike and decision record, 0x0A, 0x0E |
| The Rust rules engine | 0x03 – 0x09 |
| Database | 0x0B |
| Backend service and alerts | 0x0C, 0x0D |
| Code reviews | A `review/` exercise in every project from 0x03, plus 0x0E |
| Client meetings | An "explain it" note in every project, plus the 0x0E demo |

The format is borrowed from ALX's low-level programming track: numbered projects, small numbered tasks, a strict checker, and the same memory topics (pointers, allocation, linked lists, hash tables, libraries, files), taught the Rust way.

## The story

**Amina Wanjiru** is a ramp agent at the Nairobi station. She wants to move to cargo handling. Can she start? What's missing? Who needs to be told, and when?

That one question runs through the whole course:

| Weeks | What you build, and how it helps answer it |
| --- | --- |
| 1–3 | The pieces: her licence's countdown, her qualifications as types that can't be wrong, the staff roster loaded safely from a file |
| 4 | **The rules engine.** Given Amina's records and what Cargo needs, it answers: airside safety met, dangerous goods expiring in 13 days, forklift licence missing. Not ready yet |
| 5 | The same answer inside the TWA app, on her phone or the training office's laptop |
| 6 | Her records stored in Postgres, and read back into the engine |
| 7 | The server answers HR's transfer question, and the 06:00 job reminds Amina by WhatsApp a week before her certificate expires |
| 8 | You present her case to the training manager, and explain how TWA fits together |

Weeks 5–7 run the engine you build in week 4. If yours isn't finished, they use a ready-made one (`engine-reference/`), so you're never blocked. Each project's `Cargo.toml` shows the one line to switch to your own.

By week 8 you can run a small TWA yourself: `cargo run -p async-backend` starts the server, and `cargo run -p jobs-alerts --example morning_run` shows a morning's alerts.

## Before you start

### What you need to know already

- **How to write a small program in any language**: variables, `if`, loops, functions. *Not there yet?* Do weeks 0–3 of [CS50x](https://cs50.harvard.edu/x/), or days 1–14 of [100 Days of TypeScript](https://github.com/sagebeme/100-Days-of-TypeScript), then come back.
- **Basic use of a terminal**: changing folders, running a command. *Not there yet?* The first lecture of [The Missing Semester](https://missing.csail.mit.edu/) covers it in an hour.
- **Basic Git**: clone, commit, push. *Not there yet?* Read [Git basics](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository) chapters 2.1–2.5.

You don't need to know C, memory management or any systems programming. The course teaches those.

### What you need installed

| You need | Check you have it | If it's missing |
| --- | --- | --- |
| **Rust** (week 1 onwards) | `rustc --version` and `cargo --version` print versions | Install from [rustup.rs](https://rustup.rs/). See the Windows note below |
| **A C toolchain on Windows** (week 3 onwards) | `cargo test -p errors-files-cli` builds without "dlltool" or "link.exe" errors | Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with **Desktop development with C++**. Or, if you installed Rust's GNU toolchain, install [WinLibs](https://winlibs.com/) (`winget install BrechtSanders.WinLibs.POSIX.UCRT`) and add its `mingw64\bin` folder to your `PATH` |
| **Git** | `git --version` | [git-scm.com/downloads](https://git-scm.com/downloads) |
| **An editor that understands Rust** | Errors appear as you type | [VS Code](https://code.visualstudio.com/) with [rust-analyzer](https://rust-analyzer.github.io/), or RustRover |
| **Node.js and Tauri's prerequisites** (week 1 spike, week 5) | `node --version` prints 20 or later | [nodejs.org](https://nodejs.org/), then the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your system |
| **Android Studio** (week 1 spike, for the phone build) | Android Studio opens, and has an SDK and NDK installed | [developer.android.com/studio](https://developer.android.com/studio), then the Android section of the Tauri prerequisites |
| **Docker** (week 6) | `docker run hello-world` prints a greeting | [Get Docker](https://docs.docker.com/get-started/get-docker/), or install [PostgreSQL](https://www.postgresql.org/download/) directly |

On **macOS and Linux**, the default Rust install already includes the C tools it needs.

### Get the course

```bash
git clone https://github.com/sagebeme/learning-rust.git
cd learning-rust
cargo test -p hello-cargo
```

The first run downloads the exact Rust version pinned in `rust-toolchain.toml`. Every test then fails with "not yet implemented". That's expected: you haven't written anything yet.

## How a project works

```
0x01-values-control/
├── README.md        before you start, the story so far, what to build, what to read, how to know you're done
├── src/             your code: one file per task, each starting as todo!()
├── tests/           the checker: one test file per task
├── review/          a pull request to review, and its answer key (from 0x03)
└── solution/        look only after you've tried
```

Tasks `0`, `1`, `2`… are mandatory. Tasks numbered `100` and up are advanced: do them if you have time.

```bash
cargo test -p values-control                          # one project
cargo test -p values-control --test t00_expiry_label  # one task
cargo fmt --all                                       # format your code
cargo clippy -p values-control -- -D warnings         # lint it
```

A task is done when its tests pass **and** clippy prints nothing. CI runs the same checks against every `solution/`, with warnings treated as errors.

### Where am I?

```bash
cargo xtask progress            # every project
cargo xtask progress 0x01 0x02  # just these
```

It runs each project's tests and clippy, without changing your code, and lists every task:

```
0x01 Values and control flow — 3 of 6 done
  [x] t00   What a licence countdown means
  [ ] t02   Total hours, without overflowing        to do
  [!] t03   The four-day shift cycle                0 of 2 tests pass
  [~] t04   The duty board                          tests pass, clippy has 1 warning(s)
```

`[x]` is done, `[~]` passes its tests but clippy has something to say, `[!]` has failing tests, and `[ ]` is still `todo!()`. A project that doesn't compile shows its first error. The report is also saved to `progress.json` (ignored by git), which the course website reads.

## The 8 weeks

| Week | Projects | What you'll learn |
| --- | --- | --- |
| 1 | [0x00 hello-cargo](0x00-hello-cargo) · [0x01 values-control](0x01-values-control) · [0x02 functions-modules](0x02-functions-modules) · **[the Tauri spike](docs/week-1-tauri.md)** | Tooling, reading compiler errors, types and overflow, control flow, modules, unit tests. Setting up Tauri and calling your first Rust command from the app |
| 2 | [0x03 ownership-memory](0x03-ownership-memory) · [0x04 structs-enums-match](0x04-structs-enums-match) | Stack and heap, moves, borrowing, slices, `String` vs `&str`. Modelling qualifications so bad states can't exist |
| 3 | [0x05 errors-files-cli](0x05-errors-files-cli) · [0x06 collections-iterators](0x06-collections-iterators) · [0x07 smart-pointers-lifetimes](0x07-smart-pointers-lifetimes) | `Result` and `?`, error types, files, power-cut-safe saves. Maps, iterators, your ALX hash table rebuilt. `Box`, `Rc`, `Arc`, threads, lifetimes |
| 4 | [0x08 traits-dates-testing](0x08-traits-dates-testing) · **[0x09 rules-engine](0x09-rules-engine)** | Traits, generics, dates, property tests. The qualification engine with shared test cases: the end of the foundations |
| 5 | [0x0A tauri-core](0x0A-tauri-core) | The engine inside a Tauri app: commands, shared state, errors the screens can show, input checks, permissions, the gap check as a command |
| 6 | [0x0B databases](0x0B-databases) | Schema design, Postgres with sqlx, database errors, upserts, transactions, feeding the engine from the database |
| 7 | [0x0C async-backend](0x0C-async-backend) · [0x0D jobs-alerts](0x0D-jobs-alerts) | Async Rust, an Axum server with auth that runs the engine. The daily alert job: from the engine to planned alerts, sending once, retries, Nairobi time |
| 8 | [0x0E architecture-clients](0x0E-architecture-clients) | Architecture rules as tests, decision records, five full code reviews, the client demo |

## Each week, three kinds of work

1. **Build:** the tasks in each project.
2. **Review:** from 0x03 on, each project has a `review/` folder with a pull request that has problems planted in it. Write the review you'd leave, then compare it with the answer key. On TWA you'll review more code than you write.
3. **Explain:** a short note, written for an HR or training manager, explaining one decision from that week. No jargon. This is practice for client meetings.

## Guides you'll come back to

- [Resources](docs/resources.md): every book, tutorial and documentation page the course uses, by topic, with links
- [Glossary](docs/glossary.md): TWA words, Rust words and working words, in plain language
- [How to review a pull request](docs/code-review-guide.md)
- [Client meetings: turning requirements into rules](docs/client-meetings.md)
- [Decision records](docs/decisions/) and the [architecture page template](docs/templates/architecture.md)

## Rules that keep you learning

- **Read the compiler error before searching for it.** Rust's errors usually say exactly what's wrong and often suggest the fix. `rustc --explain E0382` gives the long version of any error code.
- **Use AI to explain, not to write.** Ask it what an error means, or why the borrow checker objects. Don't paste in code you couldn't explain line by line to your reviewer.
- **Look at `solution/` last.** If you do, close it and write your own version from memory.
- **Never run `cargo xtask apply-solutions`.** It's for CI, and it overwrites your `src/` with the reference solutions. To see the solutions pass without touching your work, run `cargo xtask verify-solutions`: it checks them in a copy under `target/`.

## How the checker is checked

CI proves both halves of every task, so you can trust a red or green result:

- `cargo xtask check-starters`: every task has a solution and tests, the starters compile, and every task has at least one test that **fails** on its `todo!()` starter. A test that passes before you've written anything would be a checker that checks nothing.
- `cargo xtask apply-solutions --ci`, then fmt, clippy and the tests: every reference solution **passes**, with warnings treated as errors.

## Progress

- [ ] Week 1: 0x00, 0x01, 0x02, Tauri spike and decision record
- [ ] Week 2: 0x03, 0x04
- [ ] Week 3: 0x05, 0x06, 0x07
- [ ] Week 4: 0x08, 0x09 (end of the foundations)
- [ ] Week 5: 0x0A, switched to your own engine
- [ ] Week 6: 0x0B
- [ ] Week 7: 0x0C, 0x0D
- [ ] Week 8: 0x0E, and the client demo
