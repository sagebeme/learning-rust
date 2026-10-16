# 0x02. Functions and modules

**Week 1** · Small functions, private helpers, modules and your own unit tests.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x01 done | `cargo test -p values-control` passes | Go back to [0x01](../0x01-values-control) |
| To know what a function and a test are | You could write a function that adds two numbers, and say how you'd check it | The Book's [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) section |

## The story so far

Amina's licences have a countdown. Now the code is growing, so it needs organising: small functions with names, private helpers nobody else can call, and your own tests next to them. And her pay, and how recently she used each skill, need working out exactly.

## What you'll be able to do

- Split work into small functions, some public and some private
- Convert safely between number types (`u32` → `i64`) and explain why `as` is risky
- Write your own unit tests next to the code, including tests for private functions

## Read first

- The Book: [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), and [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) (sections 1 and 3)
- *Rust by Example*: [Modules](https://doc.rust-lang.org/rust-by-example/mod.html), [Unit testing](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
- [Rustlings](https://rustlings.rust-lang.org/): `modules`, `tests`

**Go deeper:** the Rust Reference on [visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html), and [`From`](https://doc.rust-lang.org/std/convert/trait.From.html), the trait behind `i64::from`.

## Tasks

```bash
cargo test -p functions-modules
```

### 0. Is this a valid station code? (`t00_station_code.rs`)

Exactly three upper-case English letters. A string in Rust is UTF-8 bytes, so `"ÉBO"` is three *characters* but four *bytes*. Check `.len()` (bytes), then check each byte with `.bytes()` and `is_ascii_uppercase`. Iterators have `.all(...)`, which reads like the rule itself.

### 1. Positions in a cargo hold (`t01_pallet_positions.rs`)

Nested loops: one over rows, one over columns. Two details to get right:

- Turn a row number into a letter with `char::from(b'A' + row)`. `b'A'` is the byte value of `A`.
- Put spaces and newlines *between* items, not after them. Either check "is this the first one?" or build a `Vec` of pieces and `.join(" ")`.

### 2. A week's pay, with overtime (`t02_weekly_pay.rs`)

Write a **private** helper, `fn overtime_hours(hours: u32) -> u32`. It has no `pub`, so code outside this file can't call it. Use `saturating_sub`, which stops at 0 instead of going below it.

Then write unit tests for that helper inside the same file:

```rust
#[cfg(test)]
mod tests {
    use super::overtime_hours;

    #[test]
    fn counts_hours_past_forty() {
        assert_eq!(overtime_hours(45), 5);
    }
}
```

The tests in `tests/` can only see public functions. Tests inside the file can see everything. `#[cfg(test)]` means that code only exists when testing.

Money is in **cents** (`u64`) on purpose. Decimal numbers like `0.1` can't be stored exactly in binary, so money stored as `f64` drifts. Your payroll and the TWA invoices must never drift.

### 3. Recency and refreshers (`t03_recency.rs`)

Two small functions the TWA engine will need: "is this person still current?" and "how many days until the refresher is due?". The second can be negative, so it returns `i64` even though its inputs are `u32`.

Convert with `i64::from(x)`. It only exists where the conversion can never lose information, which is the point. `x as i64` would also work here, but `as` silently truncates when it can't fit (try `300_i32 as u8` in a scratch file), so reviewers flag it.

### 100. A stack of pallets (`t100_stack_pallets.rs`) — advanced

A right-aligned triangle, like ALX's `print_triangle`, but returned as a `String`. `" ".repeat(n)` and `.join("\n")` keep it short.

## Checkpoint

- `cargo test -p functions-modules` passes, **including your own unit tests** for `overtime_hours`
- `cargo clippy -p functions-modules -- -D warnings` prints nothing
- You can explain the difference between the tests in `tests/` and the ones inside the file

## When you're stuck

- **"function `overtime_hours` is private"**: you called it from `tests/`. Test it from inside the file instead.
- **"mismatched types: expected `i64`, found `u32`"**: Rust never converts number types for you. Use `i64::from(...)`.
- **"attempt to subtract with overflow"**: you subtracted below zero with unsigned numbers. Use `saturating_sub`, or convert to `i64` first.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
