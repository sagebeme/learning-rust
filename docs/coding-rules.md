# How we write code in this course

The same rules for every task, every week. They're the habits a reviewer on TWA will look for, so practise them from day one.

## Before you write anything

- **Say what "done" means.** Read the task and its tests in `tests/`, then write one line in your notes: "done means `expiry_label(0)` returns `"expires today"`, and the tests pass." If the task could mean two things, ask (or check the tests) before you guess.
- **Write only what the task asks.** No extra options, no "while I'm here" features. A good idea that isn't in the task goes in your notes, not in the code.

## While you write

- **Keep functions small and clear.** If one grows past about 40 lines, look for a piece that deserves its own name. Delete code you added and don't need, including commented-out code.
- **Change only what the task needs.** Given files (like `src/model.rs`) are given for a reason. Match the code around you: its names, its error types, its style.
- **Comments say why, not what.** `// Nairobi is UTC+3 all year, no daylight saving` helps. `// add one` above `x += 1` doesn't. A public function gets a `///` doc comment that says what it takes and returns, with `# Errors` (when it returns `Err`) and `# Panics` (when it can panic):

  ```rust
  /// Reads the staff roster from a CSV file.
  ///
  /// # Errors
  ///
  /// Returns `RosterError::Read` if the file can't be read, and
  /// `RosterError::Parse` for the first line that doesn't parse.
  pub fn load_roster(path: &Path) -> Result<Vec<StaffRecord>, RosterError> {
  ```

- **Don't hide errors.** In Rust, errors are values (`Result`, `Option`), so hiding one is easy. Pass it on with `?`, or handle it on purpose:

  ```rust
  let _ = fs::write(path, text);                 // bad: a full disk is silently ignored
  let text = fs::read_to_string(path).unwrap();  // bad: a missing file crashes the app
  fs::write(path, text)?;                        // good: the caller finds out
  let text = fs::read_to_string(path)?;          // good: the caller decides what to show
  ```

  `unwrap` and `expect` are for tests, and for the rare line that truly can't fail. Then use `expect` with a comment saying why. Clippy warns about `unwrap` outside tests here (see [0x05](../0x05-errors-files-cli/README.md#panic-or-result)).

## Bugs: show it, then fix it

1. Write a test that **fails because of the bug** (`cargo test -p <project> --test <file>` shows it red).
2. Fix the code until that test passes.
3. Run the whole project's tests, so you know nothing else broke.

## Tests check, they don't print

A test must `assert!` or `assert_eq!` the result. A test that only uses `println!` passes even when the code is wrong.

```rust
#[test]
fn counts_hours_past_forty() {
    assert_eq!(overtime_hours(45), 5);   // fails when the code is wrong
}
```

## Before you call it done

```bash
cargo fmt --all                                         # format
cargo clippy -p <project> --all-targets -- -D warnings  # lint, warnings count as errors
cargo test -p <project>                                 # the checker
cargo xtask progress                                    # optional: see every task at once
```

Rust's compiler is the type checker: `cargo clippy` and `cargo test` both run it, so code that builds has passed it. All three must be clean, the same as CI.

## Committing

- **Never commit secrets or generated files**: `.env`, API keys, tokens, `target/`, `progress.json`. `.gitignore` already covers these; check `git status` before you commit.
- **Add files by name** (`git add 0x01-values-control/src/t00_expiry_label.rs`), not `git add .`, so nothing slips in by accident.
- **Say what changed and why**: "Count expiry days in Nairobi time, not UTC", not "update" or "fix".
