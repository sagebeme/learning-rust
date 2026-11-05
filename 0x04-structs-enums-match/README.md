# 0x04. Structs, enums and `match`

**Week 2** · Modelling TWA's world so that impossible situations can't be written down.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x03 done | `cargo test -p ownership-memory` passes | Go back to [0x03](../0x03-ownership-memory) |
| To know the difference between `String` and `&str` | You can say which one a struct field should usually be, and why | Re-read the Book's [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html) |

## The story so far

You know who owns what. Now TWA's world needs shapes: Amina's station, the state of each of her qualifications, and qualification types that can't be created with nonsense in them. This is where the rules engine really starts.

## What you'll be able to do

- Model data with `struct` (this *and* that) and `enum` (this *or* that)
- Put data inside enum variants, and get it back out with `match`
- Make a type that can only be built through a checked constructor

## Read first

- The Book: [Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html), [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- *Rust by Example*: [Custom Types](https://doc.rust-lang.org/rust-by-example/custom_types.html), [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [Rustlings](https://rustlings.rust-lang.org/): `structs`, `enums`, `options`

**Go deeper:** the Book's chapter on [Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html), and the [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) docs (`map`, `ok_or`, `and_then` save a lot of `match`es).

## The big idea: make illegal states unrepresentable

Suppose you model a qualification's state with flags: `is_valid: bool`, `is_expired: bool`, `in_grace: bool`. That allows eight combinations, and most of them are nonsense ("valid *and* expired"). Every function has to defend against the nonsense, and one day one won't.

An enum allows exactly the states that exist, and each one carries only the data that makes sense for it. A `Valid` qualification has days left; an `Expired` one doesn't. When you `match`, the compiler makes you handle every state, so adding a new state later shows you every place that needs updating.

This is the single most useful idea for the TWA rules engine.

## Tasks

```bash
cargo test -p structs-enums-match
```

### 0. The stations (`t00_station.rs`)

Fill in `code`, `city` and `from_code` with `match`. For `from_code`, match on `code.to_ascii_uppercase().as_str()`. You can't `match` a `String` against string literals directly, but you can match a `&str`.

The `#[derive(...)]` line gives the enum abilities for free: `Debug` lets you print it with `{:?}`, `PartialEq` lets you compare with `==`, `Copy` makes it cheap to pass around like a number, and `Ord` and `Hash` let you use it as a map key in 0x06.

### 1. A qualification's status (`t01_status.rs`)

Turn three numbers into one of four states. Read the boundaries carefully: 0 days left is `ExpiringSoon` (it's still valid today), and exactly `grace_days` overdue is still `InGrace`. Build a variant with data like `QualificationStatus::Valid { days_left }`.

### 2. Methods on the status (`t02_status_methods.rs`)

An `impl` block can live in a different file from the type, as long as it's in the same crate. Use `matches!(self, QualificationStatus::Expired)` for a one-line check. For `describe`, `match` with a pattern like `ExpiringSoon { days_left: 0 }` *before* the general `ExpiringSoon { days_left }`. Arms are tried top to bottom.

Get the grammar right ("1 day", not "1 days"). Staff will read these messages every day.

### 3. A type that can't be broken (`t03_qualification_type.rs`)

The fields are private, so outside this file the only way to get a `QualificationType` is `new`, and `new` refuses bad input. Every `QualificationType` that exists anywhere in the program is therefore valid. Return `Result<QualificationType, TypeError>`: `Ok(...)` on success, `Err(...)` with the reason otherwise.

### 100. Parse a status line (`t100_parse_status_line.rs`) — advanced

`split(',')` gives an iterator of fields; take them with `.next()`. Each step can fail, and in a function returning `Option`, `?` returns `None` early: `fields.next()?.parse().ok()?`. Parsing `"-5"` as a `u32` fails, which is what you want for a staff number.

## Review it

Open [`review/README.md`](review/README.md): a PR that models qualifications with flags and strings. Write your review, then compare with `review/ANSWERS.md`.

## Explain it

A training manager asks: *"What's the difference between 'expired' and 'in grace'? Can someone in grace work?"* Write the answer you'd give, in three sentences, and say who decides how long the grace period is. (It isn't you: it's a business rule. Find out who owns it at TWA.)

## Check yourself

Answer each one before you open the answers. If you're not sure, try it in a scratch file.

1. **What prints, and what does the compiler warn about?**

   ```rust
   enum Status {
       Valid { days_left: u32 },
       Expired,
   }

   fn describe(status: &Status) -> String {
       match status {
           Status::Valid { days_left } => format!("valid for {days_left} days"),
           Status::Valid { days_left: 1 } => "valid for 1 day".to_string(),
           Status::Expired => "expired".to_string(),
       }
   }

   fn main() {
       println!("{}", describe(&Status::Valid { days_left: 1 }));
   }
   ```

2. **Will this compile?**

   ```rust
   mod catalogue {
       pub struct QualificationType {
           code: String,
           months: u32,
       }

       impl QualificationType {
           pub fn new(code: &str, months: u32) -> Option<QualificationType> {
               if code.is_empty() || months == 0 {
                   None
               } else {
                   Some(QualificationType { code: code.to_string(), months })
               }
           }
       }
   }

   fn main() {
       let _q = catalogue::QualificationType { code: String::new(), months: 0 };
   }
   ```

3. **Will this compile?**

   ```rust
   fn city(code: String) -> &'static str {
       match code {
           "NBO" => "Nairobi",
           _ => "unknown",
       }
   }
   ```

4. **What prints?**

   ```rust
   fn parse(line: &str) -> Option<(u32, String)> {
       let mut fields = line.split(',');
       let number = fields.next()?.trim().parse().ok()?;
       let code = fields.next()?.trim().to_string();
       Some((number, code))
   }

   fn main() {
       println!("{:?}", parse("48213, DG"));
       println!("{:?}", parse("-5, DG"));
       println!("{:?}", parse("48213"));
   }
   ```

5. **Will this compile?**

   ```rust
   #[derive(Debug, Clone, Copy)]
   struct Staff {
       number: u32,
       name: String,
   }
   ```

6. **Why** is `Valid { days_left }` / `Expired` better than a struct with `is_valid: bool, is_expired: bool, days_left: u32`? Name one nonsense value the struct allows.

<details>
<summary>Answers</summary>

1. It prints `valid for 1 days`, and the compiler warns `unreachable pattern` on the second arm. Arms are tried top to bottom, and the first one already matches every `Valid`. Put the specific `days_left: 1` arm first, as task 2 says.
2. No: error `E0451`, fields `code` and `months` of struct `QualificationType` are private. Outside the module the only way in is `new`, which refuses an empty code and zero months. That's task 3's whole point.
3. No: error `E0308`, mismatched types: the patterns are `&str` and `code` is a `String`. Match on `code.as_str()`.
4. `Some((48213, "DG"))`, `None`, `None`. `"-5"` doesn't parse as a `u32`, so `.ok()?` returns `None`. `"48213"` has no second field, so the second `.next()?` returns `None`.
5. No: error `E0204`, the trait `Copy` cannot be implemented for this type. A `String` owns heap memory, so copying the bytes of the struct would give two owners of the same text. Drop `Copy` and keep `Clone`, which makes the copy explicit.
6. With the enum, only real states exist, each with only the data that makes sense, and `match` makes you handle every one. The struct allows `is_valid: true, is_expired: true`, or an expired qualification with 200 `days_left`, and every function has to guess what those mean.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs` with `pub mod on_your_own;` in `src/lib.rs`, and write your tests in `tests/on_your_own.rs`.

**The brief.** Staff can ask to swap a shift. A request is pending (with the day it was made), approved (with who approved it), rejected (with a reason), or withdrawn. Model it so a request can't be approved *and* rejected, can't be approved without a name, and can't be rejected without a reason. Then give it the operations a supervisor needs:

- `ShiftCode::new(code: &str) -> Result<ShiftCode, ...>`: a shift code is two upper-case letters and a digit, like `"AM1"`
- `SwapRequest::new(shift: ShiftCode, requested_on: u32)`
- `approve(self, by: &str)`, `reject(self, reason: &str)`, `withdraw(self)`: each returns the new request or an error saying why that change isn't allowed (only a pending request can change)
- `is_open(&self) -> bool` and `describe(&self) -> String`, for example `"AM1: approved by Wanjiku"`

**Constraints**

- No `bool` fields and no status strings: the state is an `enum` with data in its variants.
- Fields are private, so the only way to make a request or a shift code is through the checked functions.
- Every `match` lists its variants: no `_` arm, so adding a state later shows every place to update.
- Empty names and reasons are refused.

**Done when**

- Tests cover each allowed change, each refused change (approving a withdrawn request, for example) and each bad shift code
- `cargo test -p structs-enums-match --test on_your_own` passes and clippy prints nothing
- You can name one bug your design makes impossible that a version with flags would allow

## Checkpoint

- `cargo test -p structs-enums-match` passes and clippy is clean
- You can name two bugs the enum in task 1 makes impossible that the flags version allows

## When you're stuck

- **"non-exhaustive patterns"**: a `match` is missing a variant. The error lists which.
- **"field `name` of struct `QualificationType` is private"**: that's the point. Use the getter.
- **"expected `&str`, found `String`" in a match**: match on `.as_str()`.
- **An error code you don't recognise?** [Common compiler errors](../docs/common-errors.md) has a tiny example and the usual fixes for each, including [E0004 and E0599](../docs/common-errors.md#e0004-non-exhaustive-patterns).
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
