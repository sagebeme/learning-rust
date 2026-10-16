# Resources

Everything the course points you to, by topic, in one place. All of it is free. Each project's README links the parts you need that week; come back here when you want to go further.

## Rust: the official material

| Resource | What it's for |
| --- | --- |
| [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the Book") | The main textbook. Every project's "Read first" points into it |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | The same topics as short runnable examples |
| [Rustlings](https://rustlings.rust-lang.org/) | Small exercises that fix broken code, topic by topic |
| [The standard library docs](https://doc.rust-lang.org/std/) | Every type and method in `std`, with examples |
| [Compiler error index](https://doc.rust-lang.org/error_codes/error-index.html) | The long explanation of every error code (`E0382`…), the same as `rustc --explain` |
| [Clippy lint list](https://rust-lang.github.io/rust-clippy/master/) | What each clippy warning means, and why |
| [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) | How to design types and functions other people will use |
| [The Cargo Book](https://doc.rust-lang.org/cargo/) | Workspaces, dependencies, features, `Cargo.toml` |
| [The Rust Reference](https://doc.rust-lang.org/reference/) | The precise rules, when the Book's explanation isn't enough |

## Memory, smart pointers and concurrency

| Resource | What it's for |
| --- | --- |
| [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) | Why linked lists are hard in Rust, and what ownership really means (0x07) |
| [Rust Atomics and Locks](https://marabos.nl/atomics/) by Mara Bos | Threads, `Mutex`, `Arc` and atomics in depth, free online |

## Async, servers and HTTP

| Resource | What it's for |
| --- | --- |
| [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/) | How async and futures work underneath |
| [Tokio tutorial](https://tokio.rs/tokio/tutorial) | The async runtime the server and the job use |
| [Axum documentation](https://docs.rs/axum) | Routing, extractors, middleware, errors |
| [Async functions in traits (Rust blog)](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html) | Why 0x0D's `AlertSender` returns `impl Future + Send` |

## Libraries used in the course

| Library | Docs |
| --- | --- |
| serde (JSON in and out) | [serde.rs](https://serde.rs/), especially [container attributes](https://serde.rs/container-attrs.html) and [enum representations](https://serde.rs/enum-representations.html) |
| thiserror (error types) | [docs.rs/thiserror](https://docs.rs/thiserror) |
| chrono (dates and times) | [docs.rs/chrono](https://docs.rs/chrono) |
| proptest (property tests) | [The proptest book](https://proptest-rs.github.io/proptest/) |
| sqlx (Postgres) | [docs.rs/sqlx](https://docs.rs/sqlx) and its [README](https://github.com/launchbadge/sqlx) |
| tempfile (temporary test files) | [docs.rs/tempfile](https://docs.rs/tempfile) |

## Tauri (the TWA app)

| Resource | What it's for |
| --- | --- |
| [Tauri 2 documentation](https://v2.tauri.app/) | Everything about the app framework |
| [Prerequisites](https://v2.tauri.app/start/prerequisites/) | What to install on Windows, macOS, Linux, Android and iOS (week 1) |
| [Calling Rust from the frontend](https://v2.tauri.app/develop/calling-rust/) | Commands, arguments, errors (0x0A) |
| [State management](https://v2.tauri.app/develop/state-management/) | Shared state for commands (0x0A) |
| [Capabilities](https://v2.tauri.app/security/capabilities/) | Which window may call what (0x0A) |
| [Calling the frontend from Rust](https://v2.tauri.app/develop/calling-frontend/) | Events (0x0A) |

## Databases

| Resource | What it's for |
| --- | --- |
| [PostgreSQL tutorial](https://www.postgresql.org/docs/current/tutorial.html) | SQL, foreign keys, transactions (0x0B) |
| [Supabase database docs](https://supabase.com/docs/guides/database/overview) | Supabase is the Postgres TWA will be hosted on |

## Alerts and messaging (TWA's providers)

| Resource | What it's for |
| --- | --- |
| [Africa's Talking SMS](https://developers.africastalking.com/docs/sms/overview) | Sending SMS in Kenya (0x0D) |
| [WhatsApp Cloud API](https://developers.facebook.com/docs/whatsapp/cloud-api) | WhatsApp business messages: templates and opt-in rules (0x0D) |
| [Firebase Cloud Messaging](https://firebase.google.com/docs/cloud-messaging) | Push notifications (0x0D) |

## Data protection

| Resource | What it's for |
| --- | --- |
| [Office of the Data Protection Commissioner, Kenya](https://www.odpc.go.ke/) | Kenya's Data Protection Act 2019 and its guidance: staff records are personal data |

## Reviews, architecture and working with clients

| Resource | What it's for |
| --- | --- |
| [Google's code review guide](https://google.github.io/eng-practices/review/) | How reviewers and authors work together (0x0E, `docs/code-review-guide.md`) |
| [Documenting Architecture Decisions](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions) by Michael Nygard | The original short essay on decision records |
| [adr.github.io](https://adr.github.io/) | Decision record templates and examples |

## Setting up your computer

| Tool | Get it |
| --- | --- |
| Rust | [rustup.rs](https://rustup.rs/) |
| Visual Studio Build Tools (Windows) | [visualstudio.microsoft.com/visual-cpp-build-tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) |
| WinLibs MinGW-w64 (Windows, alternative to the above) | [winlibs.com](https://winlibs.com/) |
| Git | [git-scm.com/downloads](https://git-scm.com/downloads) |
| VS Code | [code.visualstudio.com](https://code.visualstudio.com/) with [rust-analyzer](https://rust-analyzer.github.io/) |
| Docker (week 6) | [docs.docker.com/get-started/get-docker](https://docs.docker.com/get-started/get-docker/) |
| Node.js (week 1 spike) | [nodejs.org](https://nodejs.org/) |
| Android Studio (week 1 spike) | [developer.android.com/studio](https://developer.android.com/studio) |

## If you've never programmed before

This course assumes you can already write a simple program in some language. If you can't yet, start with one of these, then come back:

- [CS50x](https://cs50.harvard.edu/x/), Harvard's free introduction to programming, weeks 0–3
- [100 Days of TypeScript](https://github.com/sagebeme/100-Days-of-TypeScript), days 1–14: the same Nairobi world, written for complete beginners
