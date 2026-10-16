# 0x08. Traits, dates and testing

**Week 4** · Behaviour shared across types, calendar maths that matches what people mean, and tests that try hundreds of cases for you.

## What you'll be able to do

- Define a trait and implement it for several types; use them through generics or trait objects
- Do date arithmetic with `chrono` that's right at month ends and in leap years
- Write table-driven tests and property tests, and know when each one is worth it

## Read first

- The Book: **Generic Types, Traits, and Lifetimes** (generics and traits sections), **Using Trait Objects to Abstract over Shared Behavior**
- `chrono` docs: `NaiveDate`, `Months`, `Days`
- The `proptest` book: "Getting started"
- Rustlings: `generics`, `traits`

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

## Checkpoint

- `cargo test -p traits-dates-testing` passes (including your own property) and clippy is clean
- You can explain the difference between a generic function and a trait object in one sentence each

## When you're stuck

- **"the trait bound `X: ValidityRule` is not satisfied"**: `X` has no `impl ValidityRule for X`, or you passed a `Box` where a reference was expected. Try `.as_ref()`.
- **"the size for values of type `dyn ValidityRule` cannot be known"**: add `?Sized` to the generic bound, or put the trait object behind `&` or `Box`.
- **A property test fails**: read the "minimal failing input" it prints, and turn it into a normal table row so it can never come back.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
