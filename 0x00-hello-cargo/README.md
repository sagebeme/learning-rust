# 0x00. Hello, Cargo

**Week 1** · Tooling, your first functions, and a first look at memory.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| To know how to write a small program in any language | You could write FizzBuzz in some language | See "What you need to know already" in the [main README](../README.md#what-you-need-to-know-already) |
| Rust | `cargo --version` prints a version | Install from [rustup.rs](https://rustup.rs/); Windows users also read the [setup table](../README.md#what-you-need-installed) |
| An editor with rust-analyzer | Red underlines appear when you type a mistake | [VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://rust-analyzer.github.io/) |

## The story so far

Nothing yet: this is day one. Amina's transfer question will need a rules engine, an app, a database and a server. They're all built with the same tools, and this project sets them up.

## What you'll be able to do

- Build and test a Rust project with Cargo
- Read a compiler error from top to bottom and fix what it points at
- Explain why a `String` takes up 24 bytes no matter how long the text is

## Read first

- *The Rust Programming Language* (the "Book"): [Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) and [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- *Rust by Example*: [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html) and [Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)
- [Rustlings](https://rustlings.rust-lang.org/): the `intro` and `variables` exercises

**Go deeper:** the [Cargo Book's first steps](https://doc.rust-lang.org/cargo/getting-started/first-steps.html), [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html), and the [compiler error index](https://doc.rust-lang.org/error_codes/error-index.html).

## How this project works

Every task is one file in `src/`, with its tests in `tests/`. The functions start as `todo!()`, which compiles but panics when it runs, so every test fails until you write the code.

```bash
cargo test -p hello-cargo                 # every task in this project
cargo test -p hello-cargo --test t00_welcome   # just one task
```

Before you call a task done, it also has to pass the two checks CI runs on every solution:

```bash
cargo fmt --all            # formats your code; CI rejects unformatted code
cargo clippy -p hello-cargo -- -D warnings
```

Clippy is a linter: it catches code that works but isn't idiomatic, and explains why. Treat its warnings as review comments.

## Tasks

### 0. The welcome line (`t00_welcome.rs`)

`welcome("Amina")` returns `"Karibu, Amina! Welcome to TWA."`. Use `format!`, which works like `println!` but returns a `String` instead of printing it.

### 1. Station labels (`t01_station_label.rs`)

`station_label("NBO", "Nairobi")` returns `"NBO (Nairobi)"`.

### 2. How big is each type? (`t02_type_sizes.rs`)

Return each type's name and its size in bytes, using `std::mem::size_of::<T>()`. This is your first look at memory, which the whole of week 2 builds on:

- A `u8` is 1 byte and an `i64` is 8. These live directly on the **stack**: fast, fixed-size, freed automatically when the function returns.
- A `char` is 4 bytes, because it holds any Unicode character, not just English letters.
- A `String` is **24 bytes whatever its length**. Those 24 bytes are a pointer, a length and a capacity. The text itself lives on the **heap**, a separate area for data whose size isn't known in advance.
- A `&str` is 16 bytes: a pointer and a length, borrowing text that lives somewhere else.

Once your test passes, write two sentences in your notes: where does the text of a `String` live, and what is stored on the stack?

### 100. Initials for a name badge (`t100_initials.rs`) — advanced

`initials("Amina Wanjiru Otieno")` returns `"A.W.O."`. Watch for extra spaces, lower-case input, names that start with a letter like `É`, and an empty string. Hint: `split_whitespace`, `chars`, and `to_uppercase`. That last one returns more than one character for some letters, which is why it gives you an iterator.

### Compiler errors (`errors/`)

Four small files that don't compile, on purpose. See `errors/README.md`.

## Check yourself

Answer each one in your head or your notes before you open the answers. If you're not sure, try it: `rustc --edition 2024 scratch.rs` compiles a single file.

1. **Why** does a function whose body is only `todo!()` compile, even though it promises to return a `String`?
2. **What prints?**

   ```rust
   fn main() {
       let short = String::from("Hi");
       let long = "Karibu".repeat(1000);
       println!("{} {}", std::mem::size_of_val(&short), std::mem::size_of_val(&long));
       println!("{} {}", short.len(), long.len());
   }
   ```

3. **Will this compile?** If not, what's the one-character fix?

   ```rust
   fn welcome(name: &str) -> String {
       format!("Karibu, {name}!");
   }
   ```

4. **What prints?**

   ```rust
   fn main() {
       println!("{} {}", "É".len(), "É".chars().count());
       println!("{}", std::mem::size_of::<char>());
   }
   ```

5. **Spot the bug.** This works for `"Amina Wanjiru"`. Which inputs from task 100 break it, and how?

   ```rust
   fn initials(name: &str) -> String {
       let mut out = String::new();
       for part in name.split(' ') {
           let first = part.chars().next().unwrap();
           out.push(first);
           out.push('.');
       }
       out
   }
   ```

<details>
<summary>Answers</summary>

1. `todo!()` never returns: it panics. A value that can never exist is allowed to stand in for any type, so the compiler is satisfied, and you find out when the code runs ("not yet implemented"). That's why every task compiles before you've written anything.
2. `24 24`, then `2 6000`. The `String` itself (pointer, length, capacity) is 24 bytes on a 64-bit machine, however long the text is. The text is on the heap, and `.len()` tells you how many bytes of it there are.
3. No: error `E0308`, mismatched types, expected `String`, found `()`. The semicolon turns the `format!` into a statement, so the function returns nothing. Delete the semicolon (this is `errors/e03`).
4. `2 1`, then `4`. `É` takes two bytes in UTF-8 but is one character. A `char` is always 4 bytes, so it can hold any Unicode character.
5. Two spaces in a row (`"Amina  Wanjiru"`) give an empty piece, `.next()` returns `None`, and `unwrap()` panics. An empty string panics the same way. Lower-case names also stay lower-case. Use `split_whitespace()`, which skips the gaps, and `to_uppercase()`.

</details>

## On your own

No steps, no tests given, no solution: this one is yours. Put it in this project as `src/on_your_own.rs`, add `pub mod on_your_own;` to `src/lib.rs`, and write your tests in `tests/on_your_own.rs`, copying the shape of the task tests next to it.

**The brief.** Name badges for the ramp show the surname first, in capitals, then the other names as typed, then the station: `badge("Amina Wanjiru", "NBO")` returns `"WANJIRU, Amina · NBO"`. Write `pub fn badge(full_name: &str, station: &str) -> String`.

**Constraints**

- The last word of the name is the surname. Every other word stays as it was typed, in order, one space apart.
- Extra spaces anywhere in the name don't change the result.
- A one-word name is just that word in capitals: `"Baraka"` gives `"BARAKA · NBO"`.
- A name that's empty or only spaces gives `"UNKNOWN · NBO"`.
- No `unwrap()`, and no indexing into the string with `[..]`.

**Done when**

- `tests/on_your_own.rs` has at least five tests, one for each rule above plus a name with three words, and `cargo test -p hello-cargo --test on_your_own` passes
- `cargo clippy -p hello-cargo -- -D warnings` prints nothing
- Your notes say, in one sentence, where the text of the returned `String` lives

## Checkpoint

You're done with 0x00 when:

- `cargo test -p hello-cargo` passes
- `cargo clippy -p hello-cargo -- -D warnings` prints nothing
- You've fixed all four `errors/` files and written one sentence for each explaining what the compiler told you

## When you're stuck

- **"not yet implemented"** in the test output: that's `todo!()`. You haven't replaced it yet in that task.
- **"expected `String`, found `&str`"**: `"text"` is a `&str`, a borrowed piece of text. Build an owned `String` with `format!(...)`, `String::from(...)` or `.to_string()`.
- **A test fails and you can't see why**: read the `left` and `right` values in the output. `left` is what your code returned.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
