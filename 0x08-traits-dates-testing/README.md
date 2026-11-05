# 0x08. Traits, dates and testing

**Week 4** · Behaviour shared across types, calendar maths that matches what people mean, and tests that try hundreds of cases for you.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| Week 3 done | `cargo test -p smart-pointers-lifetimes` passes | Finish [0x05](../0x05-errors-files-cli)–[0x07](../0x07-smart-pointers-lifetimes) first |
| To know what an interface is in another language (helpful, not required) | You've used one in TypeScript, Java or similar | Not needed: the Book's [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) section starts from zero |

## The story so far

You can model, load and organise TWA's data. Now its most error-prone part: dates. "Valid for 24 months" means different things to different regulators, and Amina's certificates must expire on exactly the right day, whatever rule each one follows. Next week the engine is built on this.

## What you'll be able to do

- Define a trait and implement it for several types; use them through generics or trait objects
- Do date arithmetic with `chrono` that's right at month ends and in leap years
- Write table-driven tests and property tests, and know when each one is worth it

## Read first

- The Book: [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) (the generics and traits sections), and [Using Trait Objects to Abstract over Shared Behavior](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- `chrono` docs: [`NaiveDate`](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html), [`Months`](https://docs.rs/chrono/latest/chrono/struct.Months.html), [`Days`](https://docs.rs/chrono/latest/chrono/naive/struct.Days.html)
- [The proptest book](https://proptest-rs.github.io/proptest/): "Getting started"
- [Rustlings](https://rustlings.rust-lang.org/): `generics`, `traits`

**Go deeper:** [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html), and *Rust by Example* on [traits](https://doc.rust-lang.org/rust-by-example/trait.html) and [`dyn`](https://doc.rust-lang.org/rust-by-example/trait/dyn.html).

## Why dates are where engines go wrong

"Valid for 12 months" sounds simple. But what's 12 months after 29 February? Is the last day included? Is it "12 months" or "until the end of the 12th month"? Every qualification body answers these differently, and each answer is a business rule someone has to confirm. The engine's job is to make each rule explicit, named and tested, so nobody is grounded (or allowed airside) because of an off-by-one.

Two habits for the TWA engine:

- **Use `NaiveDate`** (a calendar date, no time, no time zone) for qualification dates. A licence expires on a *day*, not at an instant.
- **Never call "today" inside the rules.** Pass `today` in as a parameter. Then the same input always gives the same answer, and tests can use any date they like.

## Tasks

```bash
cargo test -p traits-dates-testing
```

### 0. Days between dates (`t00_days_between.rs`)

Subtracting two `NaiveDate`s gives a `TimeDelta`; `.num_days()` turns it into a number.

### 1. Adding months (`t01_add_months.rs`)

`date.checked_add_months(Months::new(n))` clamps to the end of the month for you, and returns `None` only past the largest date chrono can store. The test for this task is **table-driven**: a list of `(input, months, expected)` rows checked in one loop. Add a row of your own for a case you're unsure about.

### 2. Three validity rules, one trait (`t02_validity_rules.rs`)

A trait is a promise: "every type that implements `ValidityRule` can tell you `valid_until`". Implement it three times. For `EndOfMonth(n)`: go to the first day of the completion month, add `n + 1` months, then step back one day with `pred_opt()`.

The tests end with **property tests**. Instead of examples, they state facts that must always be true ("no rule ends before it starts"), and `proptest` checks them against hundreds of random dates. When one fails, proptest shrinks it to the *smallest* failing case and prints it.

### 3. Generic code (`t03_generic_checks.rs`)

Two ways to accept "any rule":

- **Generics**, `fn is_valid_on<R: ValidityRule + ?Sized>(rule: &R, ...)`: the compiler makes a copy of the function for each type. Fast, and the usual choice.
- **Trait objects**, `&[Box<dyn ValidityRule>]`: one list holding different types, looked up at run time. Use these when you genuinely need a mixed collection.

`?Sized` lets `is_valid_on` accept a `dyn ValidityRule` too.

### 100. `Display` (`t100_countdown.rs`) — advanced

Implementing `std::fmt::Display` once gives you `to_string()` and `{}` formatting everywhere. A `match` with a guard (`days if days > 1 =>`) handles the ranges.

## Your turn: a property test of your own

Add one property to `tests/t02_validity_rules.rs`. Some ideas: "`FixedDays(a + b)` is never earlier than `FixedDays(a)`", or "adding 12 months keeps the same month". Run it, and if it fails, read the shrunk case and decide whether the code or the property is wrong.

## Review it

Open [`review/README.md`](review/README.md): an expiry check with a hidden clock, decimal years and dates stored as text.

## Explain it

Ask the client (or write the question you *would* ask): *"Your Dangerous Goods certificate says 'valid 24 months'. Is that 24 months to the day, or to the end of the 24th month?"* Then write two sentences explaining to a training manager why the answer changes some staff members' expiry dates by up to 30 days.

## Check yourself

Answer each one before you open the answers. If you're not sure, try it: `chrono` is already a dependency of this project, so a test file in `tests/` can use it.

1. **What prints?**

   ```rust
   use chrono::{Months, NaiveDate};

   fn main() {
       let d = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
       println!("{}", d.checked_add_months(Months::new(1)).unwrap());
       let leap = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
       println!("{}", leap.checked_add_months(Months::new(12)).unwrap());
   }
   ```

2. **What prints?**

   ```rust
   use chrono::NaiveDate;

   fn main() {
       let today = NaiveDate::from_ymd_opt(2026, 10, 19).unwrap();
       let expiry = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
       println!("{} {}", (expiry - today).num_days(), (today - expiry).num_days());
   }
   ```

3. **Will this compile?** If not, how could you make it compile without writing `name` for every rule?

   ```rust
   trait ValidityRule {
       fn valid_until(&self, completed_day: u32) -> u32;
       fn name(&self) -> String;
   }

   struct FixedDays(u32);

   impl ValidityRule for FixedDays {
       fn valid_until(&self, completed_day: u32) -> u32 {
           completed_day + self.0
       }
   }
   ```

4. **Will this compile?**

   ```rust
   trait Rule {
       fn days(&self) -> u32;
   }

   struct Fixed(u32);

   impl Rule for Fixed {
       fn days(&self) -> u32 {
           self.0
       }
   }

   fn describe<R: Rule>(rule: &R) -> String {
       format!("{} days", rule.days())
   }

   fn main() {
       let rules: Vec<Box<dyn Rule>> = vec![Box::new(Fixed(30))];
       for rule in &rules {
           println!("{}", describe(rule.as_ref()));
       }
   }
   ```

5. **What prints?**

   ```rust
   use std::fmt;

   struct Countdown(i64);

   impl fmt::Display for Countdown {
       fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           match self.0 {
               days if days > 1 => write!(f, "{days} days left"),
               1 => write!(f, "1 day left"),
               0 => write!(f, "expires today"),
               days => write!(f, "expired {} days ago", -days),
           }
       }
   }

   fn main() {
       println!("{} | {} | {}", Countdown(2), Countdown(1), Countdown(-3));
       let text: String = Countdown(0).to_string();
       println!("{text}");
   }
   ```

6. **Why** do the rules take `today` as a parameter instead of reading the clock themselves?

<details>
<summary>Answers</summary>

1. `2024-02-29`, then `2025-02-28`. There's no 31 February, so chrono clamps to the last day of the month; 2025 isn't a leap year, so 29 February becomes the 28th. Whether that's what a qualification body means is a business rule to confirm.
2. `13 -13`. Subtracting dates gives a signed `TimeDelta`, so "days left" goes negative once the date has passed.
3. No: error `E0046`, not all trait items implemented, missing `name`. Give the trait a default body, `fn name(&self) -> String { String::from("rule") }`, and types only write their own when they need to.
4. No: error `E0277`, the size for values of type `dyn Rule` cannot be known at compilation time. A generic parameter must have a known size unless you say otherwise. Write `<R: Rule + ?Sized>` and it prints `30 days`, which is why task 3 uses `?Sized`.
5. `2 days left | 1 day left | expired 3 days ago`, then `expires today`. Implementing `Display` once gives you both `{}` and `to_string()`. The guard arm `days if days > 1` is tried first, so 1 and 0 fall through to their own arms.
6. Then the same input always gives the same answer. A test can ask about 29 February or the last day of a grace period without waiting for that date, and the app and the server can't disagree because their clocks differ.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs` with `pub mod on_your_own;` in `src/lib.rs`, and write your tests in `tests/on_your_own.rs` (`proptest` is already a dev-dependency).

**The brief.** Before a certificate expires, the training office sends a notice. Different qualification bodies want it at different times. Write a trait, `NoticeRule`, with `fn notice_on(&self, valid_until: NaiveDate) -> NaiveDate`, and three rules:

- `DaysBefore(n)`: `n` calendar days before
- `FirstOfMonthBefore(n)`: the first day of the month, `n` months before the expiry month
- `WorkingDaysBefore(n)`: `n` working days before, skipping Saturdays and Sundays

Then `pub fn earliest_notice(rules: &[Box<dyn NoticeRule>], valid_until: NaiveDate) -> Option<NaiveDate>`, and a `Display` for each rule ("10 working days before expiry").

**Constraints**

- `NaiveDate` only. Nothing reads the clock.
- No `unwrap` outside tests: work out what your rule should do at the edge of chrono's date range, and say so in a comment.
- `WorkingDaysBefore(0)` on a Saturday expiry is a decision: make one, and write it in a test name.

**Done when**

- Each rule has a table-driven test with at least one row at a month end, one in a leap year and one across a new year
- A property test checks that no rule ever gives a date after `valid_until`
- `cargo test -p traits-dates-testing --test on_your_own` passes and clippy prints nothing
- Your notes have the question you'd ask the client to confirm one of the three rules

## Checkpoint

- `cargo test -p traits-dates-testing` passes (including your own property) and clippy is clean
- You can explain the difference between a generic function and a trait object in one sentence each

## When you're stuck

- **"the trait bound `X: ValidityRule` is not satisfied"**: `X` has no `impl ValidityRule for X`, or you passed a `Box` where a reference was expected. Try `.as_ref()`.
- **"the size for values of type `dyn ValidityRule` cannot be known"**: add `?Sized` to the generic bound, or put the trait object behind `&` or `Box`.
- **A property test fails**: read the "minimal failing input" it prints, and turn it into a normal table row so it can never come back.
- **An error code you don't recognise?** [Common compiler errors](../docs/common-errors.md) has a tiny example and the usual fixes for each, including [E0277](../docs/common-errors.md#e0277-the-trait-bound-is-not-satisfied--doesnt-implement).
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
