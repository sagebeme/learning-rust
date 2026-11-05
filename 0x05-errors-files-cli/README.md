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

## Check yourself

Answer each one before you open the answers. If you're not sure, try it in a scratch file.

1. **Will this compile?**

   ```rust
   use std::fs;

   fn main() {
       let text = fs::read_to_string("roster.csv")?;
       println!("{}", text.len());
   }
   ```

2. **What prints?**

   ```rust
   fn main() {
       println!("{:?}", "48213".parse::<u32>());
       println!("{:?}", " 48213".parse::<u32>());
       println!("{:?}", "-5".parse::<u32>());
   }
   ```

3. **Will this compile?**

   ```rust
   #[derive(Debug)]
   enum RosterError {
       BadNumber(std::num::ParseIntError),
   }

   fn staff_number(field: &str) -> Result<u32, RosterError> {
       let number = field.parse::<u32>()?;
       Ok(number)
   }
   ```

4. **What prints?** And if each line's error message uses `index` as its line number, what's wrong with the message for Brian?

   ```rust
   fn main() {
       let text = "staff_number,name,station\n48213,Amina,NBO\n,Brian,MBA\n";
       for (index, line) in text.lines().enumerate().skip(1) {
           println!("{index}: {line}");
       }
   }
   ```

5. **Spot the bug.** It compiles, and the tests with a good file pass.

   ```rust
   fn load_roster(path: &Path) -> Result<Vec<StaffRecord>, RosterError> {
       let text = fs::read_to_string(path).unwrap_or_default();
       Ok(parse_roster(&text)?)
   }
   ```

6. **Why** does task 3 write to a temporary file and rename it, instead of calling `fs::write` on the real file?

<details>
<summary>Answers</summary>

1. No: error `E0277`, the `?` operator can only be used in a function that returns `Result` or `Option`. `main` returns `()`. Write `fn main() -> Result<(), std::io::Error>` and end with `Ok(())`, or handle the error with `match`.
2. `Ok(48213)`, then `Err(ParseIntError { kind: InvalidDigit })` twice. `parse` doesn't trim: the space is an invalid digit. A `u32` can't be negative, so the `-` is invalid too. Trim fields before parsing them.
3. No: error `E0277`, `?` couldn't convert the error to `RosterError`. `?` converts errors with `From`, and nothing says how to turn a `ParseIntError` into a `RosterError`. Write that `impl From`, add `#[from]` with thiserror, or use `.map_err(RosterError::BadNumber)?`.
4. `1: 48213,Amina,NBO`, then `2: ,Brian,MBA`. `enumerate` counts from 0 and the header was index 0, so Brian's line is line 3 of the file but would be reported as line 2. The HR officer opens the file at the wrong line. Report `index + 1`, as task 1 does.
5. A missing or unreadable file becomes an empty string, which parses as an empty roster. The app says "0 staff" instead of "could not read roster.csv", and nobody notices until someone isn't on the list. Return the read error, with the path: `.map_err(|source| RosterError::Read { path: path.to_path_buf(), source })?`.
6. A power cut in the middle of `fs::write` can leave half a file, and the next start-up reads garbage. Writing a separate file, syncing it to the disk, then renaming it over the old one means anyone reading sees either the whole old file or the whole new one, never half of each.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs` with `pub mod on_your_own;` in `src/lib.rs`, and write your tests in `tests/on_your_own.rs`. `tempfile` is already a dev-dependency, as in task 3's tests.

**The brief.** TWA keeps its stations in a file, `stations.csv`, with the header `code,city,opened`. Write `pub fn load_stations(path: &Path) -> Result<Vec<StationRow>, StationError>`, where `StationRow` holds the code, the city and the year it opened. A wrong station list is worse than none, so the loader refuses the whole file at the first problem and says exactly what to fix.

**Constraints**

- Your own error type with `thiserror`. Every message includes the file line number (the header is line 1). A file that can't be read names its path.
- Refuse: the wrong number of fields, a code that isn't three upper-case English letters, an empty city, a year that isn't a whole number between 1950 and 2100, and a code that appears twice. The duplicate message names both lines: `line 7: station "MBA" already appears on line 3`.
- Blank lines are skipped; spaces around fields are trimmed.
- No `unwrap`, `expect` or `panic!` outside tests.

**Done when**

- There's a test for each refusal that checks the exact message, and one for a good file
- A test with a path that doesn't exist checks the path appears in the message
- `cargo test -p errors-files-cli --test on_your_own` passes and clippy prints nothing
- Your notes have the message an HR officer would see for a duplicate, and what they'd do about it

## Checkpoint

- `cargo test -p errors-files-cli` passes and clippy is clean (no `unwrap` outside tests)
- You can explain why `save_atomically` writes to a temporary file first, without saying "atomic"

## When you're stuck

- **"the `?` operator can only be used in a function that returns `Result`"**: the function's return type must be a `Result` (or `Option`) too.
- **"`?` couldn't convert the error to `ParseError`"**: the error types don't match. Use `.map_err(...)` to build your own, or add a `#[from]` variant.
- **"cannot move out of `fields`"** with the slice pattern: match on `fields.as_slice()` with a `&` in front of the pattern.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
