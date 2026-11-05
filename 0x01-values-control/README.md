# 0x01. Values and control flow

**Week 1** · Numbers that can't silently overflow, `if`, `match` and loops.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x00 done | `cargo test -p hello-cargo` passes | Go back to [0x00](../0x00-hello-cargo) |
| To know what `if` and a loop do in any language | You could explain FizzBuzz | Weeks 0–1 of [CS50x](https://cs50.harvard.edu/x/) |

## The story so far

You can build, test and lint Rust code. Now Amina's licences need a countdown: is each one valid, expiring soon, or expired? And how many hours has she worked this week, without the total ever silently going wrong?

## What you'll be able to do

- Choose an integer type on purpose, and say what happens when a value doesn't fit
- Use `match` to cover every case, and let the compiler tell you when you've missed one
- Write `while` and `for` loops over numbers and ranges

## Read first

- The Book: [Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) (variables, data types, functions, control flow)
- *Rust by Example*: [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html), [Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)
- [Rustlings](https://rustlings.rust-lang.org/): `functions`, `if`, `primitive_types`

**Go deeper:** [`u16::checked_add`](https://doc.rust-lang.org/std/primitive.u16.html#method.checked_add) and the other `checked_`, `saturating_` and `wrapping_` methods on the same page; the Book on [integer overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow).

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

## Check yourself

Answer each one before you open the answers. If you're not sure, try it in a scratch file.

1. **Will this compile?**

   ```rust
   fn shift(day: u32) -> &'static str {
       match day % 4 {
           0 => "early",
           1 => "late",
           2 => "night",
           3 => "off",
       }
   }
   ```

2. **What prints?**

   ```rust
   fn main() {
       let a: u16 = 65_000;
       println!("{:?}", a.checked_add(535));
       println!("{:?}", a.checked_add(600));
       println!("{}", a.wrapping_add(600));
       println!("{}", a.saturating_add(600));
   }
   ```

3. **Spot the bug.** What does `check_digit(48213)` return, and what should it return?

   ```rust
   fn check_digit(mut n: u32) -> u32 {
       let mut sum = 0;
       while n > 10 {
           sum += n % 10;
           n /= 10;
       }
       sum % 10
   }
   ```

4. **Will this compile?**

   ```rust
   fn main() {
       let total = 0;
       for h in [8, 9, 7] {
           total += h;
       }
       println!("{total}");
   }
   ```

5. **What prints?**

   ```rust
   fn main() {
       println!("{:02}:{:02}", 425 / 60, 425 % 60);
       println!("[{:02}] [{:02}]", 7, 123);
   }
   ```

6. **Why** is "wrap around in release builds" worse than "panic in debug builds", even though the panic stops the program?

<details>
<summary>Answers</summary>

1. No: error `E0004`, non-exhaustive patterns, `4_u32..=u32::MAX` not covered. The compiler doesn't work out that `% 4` can only give 0 to 3, so it wants a `_` arm (task 3).
2. `Some(65535)`, `None`, `64`, `65535`. 65,535 is the largest `u16`, so adding 535 just fits. Adding 600 doesn't: `checked_add` says so with `None`, `wrapping_add` goes round to 65,600 − 65,536 = 64, and `saturating_add` stops at the maximum.
3. It returns `4`; the right answer is `8` (4 + 8 + 2 + 1 + 3 = 18). The loop stops while `n` is still 4, so the first digit is never added. The condition should be `n > 0`. Any number from 1 to 10 shows it too: `check_digit(10)` gives 0 instead of 1.
4. No: error `E0384`, cannot assign twice to immutable variable `total`. Make it `let mut total = 0;`.
5. `07:05`, then `[07] [123]`. `{:02}` pads to *at least* two digits; it never cuts a longer number.
6. A panic is loud: it stops, and says where and why. A wrapped number is quiet and wrong: someone's hours become 64 and the payroll or training record carries on with it. That's why task 2 returns `Option` and makes the caller decide.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs`, add `pub mod on_your_own;` to `src/lib.rs`, and write your tests in `tests/on_your_own.rs`, copying the shape of the task tests.

**The brief.** Ramp shifts have a 15-minute break every two full hours. Write `pub fn break_plan(start_minute: u32, length_minutes: u32) -> Option<Vec<String>>`: `start_minute` is minutes after midnight, and the result lists when each break starts as a clock time. A shift starting at 06:00 (`360`) for 8 hours (`480`) has breaks at `["08:00", "10:00", "12:00"]`: the end of the shift at 14:00 isn't a break.

**Constraints**

- Night shifts cross midnight: start `1320` (22:00), length `360` gives `["00:00", "02:00"]`.
- Return `None` if `start_minute` is 1440 or more, or the length is 0 or more than 14 hours.
- A shift shorter than two hours has no breaks: `Some(vec![])`, not `None`.
- Use `match` for the checks, a loop for the breaks, and `format!` for the times. No `as` casts.
- Write a comment saying why your arithmetic can't overflow a `u32` once the checks have passed.

**Done when**

- Your tests cover the two examples above, a shift of exactly two hours, each `None` case, and a 14-hour shift
- `cargo test -p values-control --test on_your_own` passes and clippy prints nothing

## Checkpoint

- `cargo test -p values-control` passes
- `cargo clippy -p values-control -- -D warnings` prints nothing
- In your notes, answer: why does `total_hours` return `Option<u16>` instead of `u16`? Write it for an HR manager, not a programmer: one or two sentences, no jargon.

## When you're stuck

- **"non-exhaustive patterns"**: your `match` doesn't cover every value. The error lists the ones you missed.
- **"cannot assign twice to immutable variable"**: add `mut` to the `let`.
- **"attempt to add with overflow"** panic in a test: that's the test proving a plain `+` isn't enough. Use `checked_add`.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
