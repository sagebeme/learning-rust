# Reading compiler errors

These four files don't compile, on purpose. They live outside `src/`, so they never break `cargo test`.

For each one:

1. Compile it on its own:

   ```bash
   rustc --edition 2024 errors/e01_type_mismatch.rs
   ```

2. Read the **whole** error before touching the code. The first line says what went wrong. The arrows point at where, and the `help:` lines often hand you the fix.
3. Write one sentence in your notes: what the compiler expected, and what it found instead.
4. Fix the file, and compile it again until it runs.

| File | What you'll practise |
| --- | --- |
| `e01_type_mismatch.rs` | Reading "expected X, found Y" |
| `e02_assign_twice.rs` | Variables are read-only unless you say `mut` |
| `e03_stray_semicolon.rs` | A semicolon turns a value into "nothing" |
| `e04_used_after_move.rs` | A preview of ownership (0x03): a value can only have one owner |

Error codes like `E0308` have a long explanation built in: `rustc --explain E0308`.

Done all four? Compare with `ANSWERS.md`.
