# Common compiler errors

The errors you'll meet most while doing this course, in the order the course reaches them. Each one has a tiny program that really triggers it (checked with the Rust version pinned in `rust-toolchain.toml`), what the message means in plain words, and the usual fixes.

Two habits first:

- **Read the whole error, top to bottom.** The first line says what's wrong, the arrows show where, and the `help:` lines often hand you the fix.
- **Every error code has a long explanation built in.** Run `rustc --explain E0382` (any code works).

| Code | Short message | First met in |
| --- | --- | --- |
| [E0308](#e0308-mismatched-types) | mismatched types | [0x00](../0x00-hello-cargo/errors/README.md) |
| [E0384](#e0384-cannot-assign-twice-to-immutable-variable) | cannot assign twice to immutable variable | [0x00](../0x00-hello-cargo/errors/README.md) |
| [E0382](#e0382-borrow-of-moved-value) | borrow of moved value | [0x00](../0x00-hello-cargo/errors/README.md), properly in [0x03](../0x03-ownership-memory) |
| [E0499](#e0499-cannot-borrow-as-mutable-more-than-once-at-a-time) | cannot borrow as mutable more than once at a time | [0x03](../0x03-ownership-memory), again in [0x06](../0x06-collections-iterators) |
| [E0502](#e0502-cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable) | cannot borrow as mutable because it is also borrowed as immutable | [0x03](../0x03-ownership-memory) |
| [E0106](#e0106-missing-lifetime-specifier) | missing lifetime specifier | [0x03](../0x03-ownership-memory) task 100, properly in [0x07](../0x07-smart-pointers-lifetimes) |
| [E0004](#e0004-non-exhaustive-patterns) | non-exhaustive patterns | [0x04](../0x04-structs-enums-match) |
| [E0599](#e0599-no-method-named--found) | no method named `…` found | [0x04](../0x04-structs-enums-match) |
| [E0597](#e0597-does-not-live-long-enough) | does not live long enough | [0x07](../0x07-smart-pointers-lifetimes) |
| [E0373](#e0373-closure-may-outlive-the-current-function) | closure may outlive the current function | [0x07](../0x07-smart-pointers-lifetimes) |
| [E0277](#e0277-the-trait-bound-is-not-satisfied--doesnt-implement) | the trait bound is not satisfied / doesn't implement | [0x08](../0x08-traits-dates-testing) |

To try one, save the sample as `e.rs` and run `rustc --edition 2024 e.rs`.

## E0308: mismatched types

```rust
fn days_left() -> u32 {
    "30"
}

fn main() {
    println!("{}", days_left());
}
```

**What it means:** the code promised one type and gave another. Here the function says it returns a `u32` (a number) but hands back `"30"` (text). Rust never converts between types silently. The message says "expected `u32`, found `&str`": read both halves.

**Usual fixes:**

- Give it the right type: `30` instead of `"30"`.
- Convert on purpose: `"30".parse::<u32>()` returns a `Result`, because text might not be a number (0x05).
- **"expected `u32`, found `()`"** is the sneaky version: a stray `;` after the last expression threw the value away. Delete the semicolon (`0x00/errors/e03`).

## E0384: cannot assign twice to immutable variable

```rust
fn main() {
    let total = 0;
    total = total + 8;
    println!("{total}");
}
```

**What it means:** variables are read-only unless you say otherwise. `total` was set once and can't change.

**Usual fixes:**

- If it really must change, `let mut total = 0;`. The `help:` line suggests exactly that.
- Often clearer: make a new value instead, `let total = total + 8;` (that's called shadowing).

## E0382: borrow of moved value

```rust
fn print_badge(name: String) {
    println!("Badge: {name}");
}

fn main() {
    let name = String::from("Amina Wanjiru");
    print_badge(name);
    println!("{name}");
}
```

**What it means:** passing a `String` to a function *moves* it: the function becomes its owner and frees it when it finishes. After the move, `name` in `main` is empty-handed, so using it again is refused. (It's "use of moved value" when you use it by value, "borrow of moved value" when you only read it, as here.)

**Usual fixes:**

- Lend it instead of giving it away: `fn print_badge(name: &str)` and call `print_badge(&name)`. This is usually the right answer.
- Give the function its own copy: `print_badge(name.clone())`. Fine for one name, wasteful in a loop over 800 staff.
- If you moved it into a loop (`for n in names`), loop over a borrow instead: `for n in &names`.

Small `Copy` types (`u32`, `bool`, `char`) never move, so you'll only see this with heap-owning types like `String` and `Vec`.

## E0499: cannot borrow as mutable more than once at a time

```rust
fn main() {
    let mut roster = vec![String::from("Amina")];
    let first = &mut roster;
    let second = &mut roster;
    first.push(String::from("Brian"));
    second.push(String::from("Chebet"));
}
```

**What it means:** only one `&mut` to a value may be alive at a time. A `&mut` is alive from where it's made to where it's *last used*, so `first` is still alive when `second` is made.

**Usual fixes:**

- Finish with one before making the next: use `first` and stop, then make `second`.
- Usually you don't need two: keep one `&mut` and do everything through it. In 0x06's hash table that's `let chain = &mut self.buckets[index];`, then work only with `chain`.
- To change two different items of one slice, `split_at_mut` hands back two `&mut` that the compiler knows don't overlap.

## E0502: cannot borrow as mutable because it is also borrowed as immutable

```rust
fn main() {
    let mut codes = vec![String::from("AVSEC"), String::from("DG")];
    let first = &codes[0];
    codes.push(String::from("FORK"));
    println!("{first}");
}
```

**What it means:** "many readers or one writer, never both". `first` is a reader pointing into `codes`, and `push` is a writer. Pushing can move the whole list to a bigger block of memory, which would leave `first` pointing at freed memory: in C, a dangling pointer. Rust refuses to compile it.

**Usual fixes:**

- Finish reading before writing: move the `println!` above the `push`.
- Keep your own copy if you need it afterwards: `let first = codes[0].clone();`.
- Inside `retain` or a loop over `codes`, don't look back into `codes` while it's changing: keep a separate record (0x03 task 3 uses a `HashSet` of codes seen so far).

## E0106: missing lifetime specifier

```rust
fn longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("NBO", "MBA"));
}
```

**What it means:** the function returns a reference, and with two reference inputs the compiler can't tell which one the result borrows from. It needs to know, so it can check the caller keeps that input alive long enough.

**Usual fixes:**

- Name the connection: `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`. That says "the result lives as long as both inputs do".
- If the result only ever comes from one input, tie it to that one: `fn first_word<'a>(line: &'a str, sep: &str) -> &'a str`.
- If the result is newly built text, it isn't a borrow at all: return `String`.

With only one reference input, or a `&self` method, the compiler fills the lifetime in by itself, which is why you rarely see this before 0x07.

## E0004: non-exhaustive patterns

```rust
enum Status {
    Valid,
    Expiring,
    Expired,
}

fn label(status: Status) -> &'static str {
    match status {
        Status::Valid => "valid",
        Status::Expiring => "expiring soon",
    }
}

fn main() {
    println!("{}", label(Status::Expired));
}
```

**What it means:** a `match` must handle every possible value, and this one forgot `Status::Expired`. The error lists exactly which variants are missing. This is a feature: add a variant to an enum later, and the compiler shows you every `match` that needs updating.

**Usual fixes:**

- Add the missing arm: `Status::Expired => "expired",`.
- Use `_ =>` only when "everything else" really means the same thing. On your own enums, prefer listing each variant, so new ones aren't silently swallowed.

## E0599: no method named `…` found

```rust
#[derive(Debug)]
enum Status {
    Valid,
}

fn main() {
    let a = Status::Valid;
    let b = a.clone();
    println!("{a:?} {b:?}");
}
```

**What it means:** the type has no method by that name. Most often the method comes from a trait the type doesn't implement yet (here `Clone`), or a trait that isn't imported. Sometimes it's just a typo, or you're calling it on the wrong type (an `Option<Staff>` instead of a `Staff`).

**Usual fixes:**

- Derive or implement the trait: `#[derive(Debug, Clone)]`. The `note:` under the error names the trait.
- Bring the trait into scope with `use`: a trait's methods only work where the trait is imported (for example `use std::io::Write;` for `write_all`).
- Check the type you actually have: unwrap the `Option` or `Result` first, with `?`, `match` or `if let`.

## E0597: does not live long enough

```rust
fn main() {
    let station: &str;
    {
        let code = String::from("NBO");
        station = &code;
    }
    println!("{station}");
}
```

**What it means:** `station` borrows `code`, but `code` is freed at the closing `}`, while `station` is still used after it. A borrow can't outlive its owner.

**Usual fixes:**

- Move the owner out to where it's needed: declare `code` in the outer scope.
- Keep an owned value instead of a borrow: make `station` a `String` and move `code` into it.
- In a struct that holds a `&'a str` (0x07's name index), the data it borrows from must be created first and live longer than the struct.

## E0373: closure may outlive the current function

```rust
use std::thread;

fn main() {
    let names = vec![String::from("Amina")];
    let handle = thread::spawn(|| {
        println!("{}", names.len());
    });
    handle.join().unwrap();
}
```

**What it means:** a new thread might keep running after `main` ends, so it can't borrow `names`, which `main` owns. (You know `join` waits, but `thread::spawn` can't rely on that.)

**Usual fixes:**

- Hand the data over to the thread: `thread::spawn(move || ...)`. The `help:` line suggests it.
- Several threads need the same data: wrap it in `Arc`, and `move` an `Arc::clone` into each (0x07 task 3).
- Threads that must finish before the function returns: `std::thread::scope`, whose threads *can* borrow.

## E0277: the trait bound is not satisfied / doesn't implement

```rust
struct Station {
    code: String,
}

fn main() {
    let nbo = Station { code: String::from("NBO") };
    println!("{}", nbo);
    let _ = nbo.code;
}
```

**What it means:** some code needs a type to have an ability (a trait) that it doesn't have. Here `{}` needs `Display`, and `Station` has none. With your own traits it reads "the trait bound `X: ValidityRule` is not satisfied": a generic function asked for `T: ValidityRule`, and you passed something without `impl ValidityRule`.

**Usual fixes:**

- To print for debugging, `#[derive(Debug)]` and `{:?}`.
- To print for people, write `impl std::fmt::Display for Station` (0x08 task 100).
- For your own trait, add the `impl` for that type, or pass a type that has one. With a `Box<dyn Trait>`, try `.as_ref()` or `&*boxed` where a `&dyn Trait` is expected.
- For `?` in a function returning your own error type, it's a missing `From` conversion: `#[from]` with `thiserror` writes it (0x05).

## Still stuck?

- [The Rust compiler error index](https://doc.rust-lang.org/error_codes/error-index.html): every code, with examples.
- The Book on [references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) and [lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).
- Each project's "When you're stuck" section, for the errors that project's tasks tend to cause.
