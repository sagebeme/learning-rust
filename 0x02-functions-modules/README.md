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

## Check yourself

Answer each one before you open the answers. If you're not sure, try it in a scratch file.

1. **Will this compile?**

   ```rust
   mod pay {
       fn overtime_hours(hours: u32) -> u32 {
           hours.saturating_sub(40)
       }

       pub fn paid_hours(hours: u32) -> u32 {
           hours + overtime_hours(hours)
       }
   }

   fn main() {
       println!("{}", pay::paid_hours(45));
       println!("{}", pay::overtime_hours(45));
   }
   ```

2. **What prints?**

   ```rust
   fn main() {
       println!("{} {}", 300_i32 as u8, -1_i32 as u32);
       println!("{} {}", 38_u32.saturating_sub(40), 45_u32.saturating_sub(40));
   }
   ```

3. **Will this compile?**

   ```rust
   fn main() {
       let a = i64::from(7_u32);
       let b = u32::from(7_i64);
       println!("{a} {b}");
   }
   ```

4. **Spot the bug.** What do these three calls return, and which one is wrong for TWA's station codes?

   ```rust
   pub fn is_station_code(code: &str) -> bool {
       code.chars().count() == 3 && code.chars().all(|c| c.is_uppercase())
   }

   // is_station_code("NBO"), is_station_code("ÉBO"), is_station_code("NB0")
   ```

5. **Why** can the tests inside `t02_weekly_pay.rs` call `overtime_hours`, when the tests in `tests/t02_weekly_pay.rs` can't?

<details>
<summary>Answers</summary>

1. No: error `E0603`, function `overtime_hours` is private. `paid_hours` can call it because it's in the same module; `main` is outside. Delete the last line and it prints `50`.
2. `44 4294967295`, then `0 5`. `as` doesn't check: 300 doesn't fit in a `u8`, so it keeps the low 8 bits (300 − 256 = 44), and −1 becomes the largest `u32`. `saturating_sub` stops at 0 instead of going below it.
3. No: error `E0277`, the trait bound `u32: From<i64>` is not satisfied. `From` only exists where nothing can be lost, and an `i64` can be negative or too big. For that direction you'd use `u32::try_from(x)`, which returns a `Result`.
4. `true`, `true`, `false`. `"ÉBO"` passes because `É` is an upper-case letter and three *characters*, but it isn't a station code. The rule is three upper-case *English* letters: check the bytes with `is_ascii_uppercase`, as task 0 says.
5. Files in `tests/` are compiled as a separate crate that uses yours from the outside, so they see only `pub` items. A `#[cfg(test)] mod tests` inside the file is a child module, and child modules can see their parent's private items through `use super::...`.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs` with `pub mod on_your_own;` in `src/lib.rs`.

**The brief.** Shift pay depends on the day: weekdays are paid at the normal rate, Saturdays at one and a half times, Sundays at double. Write `pub fn shift_pay_cents(hours: u32, rate_cents: u64, day: u32) -> Option<u64>`, where `day` is 0 for Monday through 6 for Sunday.

**Constraints**

- Money is whole cents in a `u64`. No floats anywhere.
- Half a cent rounds down: 3 hours at 333 cents on a Saturday is 1,498 cents. Multiply before you divide.
- `None` for a `day` above 6, for more than 24 hours, and for any calculation that would overflow (use the `checked_` methods).
- At least two private helper functions, each with its own unit tests inside the file. Convert with `u64::from`, never `as`.
- The public function is tested from `tests/on_your_own.rs`.

**Done when**

- `cargo test -p functions-modules` runs both your inside-the-file tests and `tests/on_your_own.rs`, and they pass
- You have a test for each day type, the rounding case, each `None` case, and 0 hours
- Clippy prints nothing

## Checkpoint

- `cargo test -p functions-modules` passes, **including your own unit tests** for `overtime_hours`
- `cargo clippy -p functions-modules -- -D warnings` prints nothing
- You can explain the difference between the tests in `tests/` and the ones inside the file

## When you're stuck

- **"function `overtime_hours` is private"**: you called it from `tests/`. Test it from inside the file instead.
- **"mismatched types: expected `i64`, found `u32`"**: Rust never converts number types for you. Use `i64::from(...)`.
- **"attempt to subtract with overflow"**: you subtracted below zero with unsigned numbers. Use `saturating_sub`, or convert to `i64` first.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
