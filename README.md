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

## Setup

1. Install Rust with [rustup](https://rustup.rs). On Windows, the GNU toolchain works without installing Visual Studio:

   ```bash
   rustup-init --default-host x86_64-pc-windows-gnu
   ```

2. Clone this repo and run the tests once. The first run downloads the exact Rust version pinned in `rust-toolchain.toml`:

   ```bash
   git clone https://github.com/sagebeme/learning-rust.git
   cd learning-rust
   cargo test -p hello-cargo
   ```

   Every test fails at first, with "not yet implemented". That's expected: you haven't written anything yet.

3. Install an editor extension: **rust-analyzer** for VS Code, or RustRover. It shows compiler errors as you type.

## How a project works

```
0x01-values-control/
├── README.md        what to build, what to read, and how to know you're done
├── src/             your code: one file per task, each starting as todo!()
├── tests/           the checker: one test file per task
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

## The 8 weeks

| Week | Projects | What you'll learn |
| --- | --- | --- |
| 1 | [0x00 hello-cargo](0x00-hello-cargo) · [0x01 values-control](0x01-values-control) · [0x02 functions-modules](0x02-functions-modules) · **[the Tauri spike](docs/week-1-tauri.md)** | Tooling, reading compiler errors, types and overflow, control flow, modules, unit tests. Setting up Tauri and calling your first Rust command from the app |
| 2 | 0x03 ownership-memory · 0x04 structs-enums-match | Stack and heap, moves, borrowing, slices, `String` vs `&str`, `Drop`. Modelling qualifications |
| 3 | 0x05 errors-files-cli · 0x06 collections-iterators · 0x07 smart-pointers-lifetimes | `Result` and `?`, error types, files, safe writes. Maps, iterators, a hand-built hash map. `Box`, `Rc`, `Arc`, lifetimes |
| 4 | 0x08 traits-dates-testing · **0x09 rules-engine** | Traits, generics, dates, property tests. A small working qualification engine: the end of the foundations |
| 5 | 0x0A tauri-core | The engine inside a Tauri app: commands, shared state, events, errors the screens can show, permissions, desktop and mobile builds |
| 6 | 0x0B databases | SQL design, Postgres with sqlx, migrations, transactions, SQLite on devices |
| 7 | 0x0C async-backend · 0x0D jobs-alerts | Async Rust, an Axum web service, the daily expiry job, email, SMS, WhatsApp and push alerts |
| 8 | 0x0E architecture-and-clients | Architecture decisions, reviewing real pull requests, presenting to a client |

Weeks 2–8 are added to this repo ahead of when you need them.

## Each week, three kinds of work

1. **Build:** the tasks in each project.
2. **Review:** from 0x03 on, each project has a `review/` folder with a pull request that has problems planted in it. Write the review you'd leave, then compare it with the answer key. On TWA you'll review more code than you write.
3. **Explain:** a short note, written for an HR or training manager, explaining one decision from that week. No jargon. This is practice for client meetings.

## Rules that keep you learning

- **Read the compiler error before searching for it.** Rust's errors usually say exactly what's wrong and often suggest the fix. `rustc --explain E0382` gives the long version of any error code.
- **Use AI to explain, not to write.** Ask it what an error means, or why the borrow checker objects. Don't paste in code you couldn't explain line by line to your reviewer.
- **Look at `solution/` last.** If you do, close it and write your own version from memory.
- **Never run `cargo xtask apply-solutions`.** It's for CI, and it overwrites your `src/` with the reference solutions.

## Progress

- [ ] Week 1: 0x00, 0x01, 0x02, Tauri spike and decision record
- [ ] Week 2: 0x03, 0x04
- [ ] Week 3: 0x05, 0x06, 0x07
- [ ] Week 4: 0x08, 0x09 (end of the foundations)
- [ ] Week 5: 0x0A
- [ ] Week 6: 0x0B
- [ ] Week 7: 0x0C, 0x0D
- [ ] Week 8: 0x0E
