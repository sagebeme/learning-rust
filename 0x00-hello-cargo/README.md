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
