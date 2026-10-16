# 0x05. Errors, files and the command line

**Week 3** · Failing well: errors that say what went wrong and where, and files that are never half-written.

ALX's file I/O and argc/argv projects, the Rust way. In C you checked every return value (or forgot to) and closed every file descriptor (or leaked it). In Rust, a function that can fail *says so in its type*, and files close themselves when they go out of scope.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| Week 2 done | `cargo test -p structs-enums-match` passes | Finish [0x03](../0x03-ownership-memory) and [0x04](../0x04-structs-enums-match) first |
| To know `Option` and `match` | You can explain what `?` did in 0x04's task 100 | Re-read 0x04's task 100, and the Book's [Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) section |
| **On Windows: a C toolchain** | `cargo test -p errors-files-cli` builds (it fails tests, but compiles) | This is the first project that needs one: see the [setup table](../README.md#what-you-need-installed) |

## The story so far

TWA's types are taking shape. But real data arrives from outside: HR exports a roster file, and it will have mistakes in it. Amina's records have to be read from it, and saved again, without a bad line or a power cut ever losing anything.

## What you'll be able to do

- Design an error type that tells the person reading it exactly what to fix
- Use `Result` and `?` so errors flow upwards without a pile of `if` checks
- Read and write files safely, including a save that survives a power cut

## Read first

- The Book: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) (all of it, especially [To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html))
- *Rust by Example*: [Error handling](https://doc.rust-lang.org/rust-by-example/error.html), especially [Result](https://doc.rust-lang.org/rust-by-example/error/result.html) and [Defining an error type](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html)
- The [`thiserror` docs](https://docs.rs/thiserror) (one page)
- [Rustlings](https://rustlings.rust-lang.org/): `error_handling`

**Go deeper:** [`std::fs`](https://doc.rust-lang.org/std/fs/index.html) and [`File::sync_all`](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all); the [`clap`](https://docs.rs/clap) crate for real command-line tools; the [Error Handling chapter of the Rust API Guidelines](https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err).

## Panic or `Result`?

- **`Result`** for anything that can go wrong in normal use: a missing file, a bad line in a CSV, a network that's down. The caller decides what to do.
- **`panic!`** (and `unwrap`, `expect`) only for "this can't happen unless the program itself has a bug". In the TWA engine and server, a panic is an outage. That's why this repo's clippy settings warn about `unwrap` outside tests.

## Tasks

```bash
cargo test -p errors-files-cli
```

`src/model.rs` is given: the record type and the error types. Read it first. `#[derive(thiserror::Error)]` and `#[error("...")]` write the error messages for you.

### 0. Parse one line (`t00_parse_line.rs`)

Return `Ok(StaffRecord)` or the *first* problem as `Err(ParseError::...)`. Two handy tools:

- `let &[a, b, c] = fields.as_slice() else { return Err(...) };` takes exactly three fields, or bails out.
- `.parse::<u32>().map_err(|_| ParseError::BadStaffNumber { ... })?` turns the library's error into yours, then `?` returns it.

Every error carries the line number. "Unknown station" is useless to the HR officer fixing a 900-line export. "Line 412: unknown station "LHR"" is a two-second fix.

### 1. Parse the whole roster (`t01_parse_roster.rs`)

`.lines().enumerate().skip(1)` gives you each line with its index, minus the header. Use `?` on each parsed line: the first error stops the loop and is returned.

### 2. Load from a file (`t02_load_roster.rs`)

`fs::read_to_string` returns an `io::Error` that doesn't say *which* file. Add the path yourself with `.map_err(|source| RosterError::Read { path: ..., source })?`. The parse step needs no `map_err`: `#[from]` on the `Parse` variant tells `?` how to convert.

Notice there's no `close()`. The file is closed the moment the value holding it goes out of scope, even on an early `return`. That's ALX's leaked-file-descriptor bug, gone.

### 3. A save that survives a power cut (`t03_save_atomically.rs`)

Stations lose power. If the app is halfway through writing a file when that happens, a plain `fs::write` can leave half a file, and the next start-up reads garbage. The fix used by real apps (storytold's `craft-atomic` crate does exactly this):

1. write everything to a temporary file next to the real one
2. `sync_all()`, so the data is really on the disk and not just in a memory cache
3. `fs::rename` the temporary file over the real one

A rename inside one folder is atomic: anyone reading sees the whole old file or the whole new one. This is the pattern the TWA app will use for its local data and settings.

### 100. Command-line arguments (`t100_parse_args.rs`) — advanced

Hand-parse `check <file>` and `count <file> [--station CODE]`. Turning the argument list into an iterator and calling `.next()` keeps it readable. Real tools use the `clap` crate, which writes all of this (and `--help`) for you. Do it by hand once so you know what it's doing.

## Review it

Open [`review/README.md`](review/README.md): a PR that loads station data with `unwrap` everywhere. Write your review, then compare with `review/ANSWERS.md`.

## Explain it

An HR officer uploads a roster and the app says: `line 412: unknown station "LHR"`. Write the two-sentence explanation you'd give them of what happened and what to do. Then write the version a *bad* error message would have forced them to send to IT instead.

## Checkpoint

- `cargo test -p errors-files-cli` passes and clippy is clean (no `unwrap` outside tests)
- You can explain why `save_atomically` writes to a temporary file first, without saying "atomic"

## When you're stuck

- **"the `?` operator can only be used in a function that returns `Result`"**: the function's return type must be a `Result` (or `Option`) too.
- **"`?` couldn't convert the error to `ParseError`"**: the error types don't match. Use `.map_err(...)` to build your own, or add a `#[from]` variant.
- **"cannot move out of `fields`"** with the slice pattern: match on `fields.as_slice()` with a `&` in front of the pattern.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
