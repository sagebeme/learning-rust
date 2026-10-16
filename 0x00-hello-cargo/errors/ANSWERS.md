# Answers

Try every file yourself before reading these.

**e01_type_mismatch.rs** — `E0308`, mismatched types. The function promises a `u32` but returns the text `"30"`. Text and numbers are different types, and Rust never converts between them silently. Fix: return `30`.

**e02_assign_twice.rs** — `E0384`, cannot assign twice to an immutable variable. `let total = 0;` can't change afterwards. Fix: `let mut total = 0;`. The compiler's `help:` line suggests exactly that.

**e03_stray_semicolon.rs** — `E0308` again, but with a twist: expected `u32`, found `()`. The `if`/`else` is an expression that produces the number, but the `;` after it throws the number away, so the function ends with "nothing" (`()`). Fix: delete that semicolon. The last expression in a function, with no semicolon, is its return value.

**e04_used_after_move.rs** — `E0382`, borrow of moved value. Calling `print_badge(name)` hands ownership of the `String` to the function, which drops it when it finishes, so `name` can't be used afterwards. Two fixes:

- Let the function borrow instead of own: `fn print_badge(name: &str)` and call `print_badge(&name)`. This is usually the right answer.
- Give it a copy: `print_badge(name.clone())`. It works, but it copies the text on the heap. Fine for one name, wasteful in a loop over 800 staff.

You'll learn why in 0x03.
