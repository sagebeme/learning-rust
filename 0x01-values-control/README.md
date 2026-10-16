# 0x01. Values and control flow

**Week 1** · Numbers that can't silently overflow, `if`, `match` and loops.

## What you'll be able to do

- Choose an integer type on purpose, and say what happens when a value doesn't fit
- Use `match` to cover every case, and let the compiler tell you when you've missed one
- Write `while` and `for` loops over numbers and ranges

## Read first

- The Book: **Common Programming Concepts** (variables, data types, functions, control flow)
- *Rust by Example*: **Primitives**, **Flow of Control**
- Rustlings: `functions`, `if`, `primitive_types`

## Tasks

```bash
cargo test -p values-control
```

### 0. What a licence countdown means (`t00_expiry_label.rs`)

Turn "days left" into one of `"expired"`, `"expires today"`, `"expiring soon"` (1 to 30 days) or `"valid"`. Use `match` with ranges: `1..=30` means 1 up to and including 30. A `match` has to cover every possible value, so think about negative numbers and very large ones.

This is the first rule of the TWA engine in miniature. The real one will have grace periods, but the shape is the same.

### 1. A staff number's check digit (`t01_check_digit.rs`)

Add up the digits and keep the last digit of the sum. Peel digits off with `% 10` (the last digit) and `/ 10` (drop it), in a `while` loop. You'll need a variable you can change, which in Rust has to be declared `let mut`.

### 2. Total hours, without overflowing (`t02_total_hours.rs`)

A `u16` holds 0 to 65,535. What happens when a sum goes past that?

- In a debug build (`cargo test`), Rust **panics**: the program stops with an error.
- In a release build, it **wraps around** silently to a small, wrong number.

Neither is acceptable for payroll or training hours. `a.checked_add(b)` returns `Some(sum)` if it fits and `None` if it doesn't, so the caller has to deal with it. The `?` operator returns `None` early for you: `total = total.checked_add(h)?;`

This matters more than it looks. In C, the same mistake goes unnoticed. Your ALX binary search had exactly this bug with unsigned numbers going below zero, and so would Rust code using plain `-` on a `usize` that reaches 0.

### 3. The four-day shift cycle (`t03_shift_for_day.rs`)

`day % 4` gives 0, 1, 2 or 3. `match` on it. You'll find the compiler insists on a final `_` arm even though `% 4` can only give four values. It can't prove that, so it makes you handle "anything else".

### 4. The duty board (`t04_duty_board.rs`)

The classic FizzBuzz, with ground-handling words. Build a `Vec<String>` with `Vec::new()` and `push`. Try matching on a pair, `match (slot % 3, slot % 5)`, so all four cases sit side by side.

### 100. Minutes as a clock reading (`t100_minutes_to_clock.rs`) — advanced

`425` minutes is `"07:05"`. `format!("{:02}", n)` pads a number to at least two digits.

## Checkpoint

- `cargo test -p values-control` passes
- `cargo clippy -p values-control -- -D warnings` prints nothing
- In your notes, answer: why does `total_hours` return `Option<u16>` instead of `u16`? Write it for an HR manager, not a programmer: one or two sentences, no jargon.

## When you're stuck

- **"non-exhaustive patterns"**: your `match` doesn't cover every value. The error lists the ones you missed.
- **"cannot assign twice to immutable variable"**: add `mut` to the `let`.
- **"attempt to add with overflow"** panic in a test: that's the test proving a plain `+` isn't enough. Use `checked_add`.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
