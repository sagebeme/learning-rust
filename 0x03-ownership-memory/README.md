# 0x03. Ownership and memory

**Week 2** · The idea the rest of Rust is built on: every value has exactly one owner.

In C (your ALX projects), *you* decided when memory was allocated and freed, and the compiler trusted you. Every `malloc` needed a matching `free`, at the right moment, exactly once. Rust makes the same decisions, but the compiler checks them before the program ever runs.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| Week 1 done | `cargo test -p functions-modules` passes | Finish [0x00](../0x00-hello-cargo)–[0x02](../0x02-functions-modules) first |
| 0x00's stack-and-heap task | You can say where a `String`'s text lives | Re-read task 2 of [0x00](../0x00-hello-cargo) |
| Your ALX C code (optional, for the review) | You remember what `malloc` and `free` did | Not needed: the review explains each bug |

## The story so far

You can write and organise small programs. But every one of them copied data freely. TWA's engine will handle 800 staff and their records on cheap phones, so it has to be precise about who owns each piece of data and when it's freed. That's what makes Rust different from every language you've used, and the rest of the course rests on it.

## What you'll be able to do

- Say who owns a value at any point in a function, and when it gets freed
- Choose between taking ownership (`String`), borrowing to read (`&str`, `&[T]`) and borrowing to change (`&mut`)
- Return references and slices that point into the caller's data, without copying it

## Read first

- The Book: [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) (all three sections). Read it twice; it's that important.
- *Rust by Example*: [Scoping rules](https://doc.rust-lang.org/rust-by-example/scope.html): RAII, ownership and moves, borrowing
- [Rustlings](https://rustlings.rust-lang.org/): `move_semantics`, `strings`, `vecs`

**Go deeper:** the [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and [`str`](https://doc.rust-lang.org/std/primitive.str.html) docs (which methods return slices, and which allocate), and [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/), the introduction.

## The rules, in plain words

1. **Every value has one owner.** When the owner goes out of scope, the value is freed automatically. This is Rust's `free()`, inserted for you, in exactly the right place.
2. **Assigning or passing a `String` moves it.** The old variable can't be used afterwards. That's error `E0382` from `0x00/errors/e04`.
3. **You can borrow instead of moving:**
   - **Many readers**, `&T`: as many as you like, at the same time.
   - **Or one writer**, `&mut T`: exactly one, and nobody else may read while it exists.

   Never both at once. That one rule rules out a whole family of C bugs: changing a list while something else is reading it.
4. **A borrow can't outlive its owner.** You can't return a reference to something that's about to be freed. In C that's a dangling pointer. In Rust it doesn't compile.

**Copy types** (`u32`, `i64`, `bool`, `char`) are small and live on the stack, so assigning them just copies the value and nothing moves.

## Tasks

```bash
cargo test -p ownership-memory
```

### 0. The longest name, borrowed (`t00_longest_name.rs`)

Return `Option<&str>` pointing at a name *inside* the list. The test checks the memory address, so a copy won't pass. Count characters with `.chars().count()`, not `.len()`, because `.len()` counts bytes.

### 1. Tidy a name in place (`t01_tidy_name.rs`)

You get `&mut String`, a mutable borrow. Build the tidy version, then replace the original with `*name = tidy;`. The `*` means "the thing this reference points at".

### 2. Split a name without copying (`t02_split_name.rs`)

Return two `&str` slices of the input. A slice is a pointer and a length into memory someone else owns, so it costs nothing to make. `trim` and `split_once` both return slices.

### 3. Remove duplicates in place (`t03_dedupe_codes.rs`)

`Vec::retain` keeps the items for which your closure returns `true`. You need to remember which codes you've seen, and you'll hit the borrow checker if you try to look *back into `codes`* while `retain` is changing it. Keep a separate record of what you've seen: a `HashSet` is ideal, and `insert` returns `false` when the item was already there.

### 4. Badge lines, taking ownership (`t04_badge_lines.rs`)

This function takes `Vec<String>`, not `&[String]`: the caller hands the whole list over. Because you own the strings, you can change them and hand them back instead of allocating new ones. `into_iter()` gives you each `String` by value. Compare it with `iter()`, which only lends them.

### 100. Teams without copying (`t100_chunk_roster.rs`) — advanced

The `'a` in the signature is a **lifetime**: it says the names in the result borrow from the same place as the names in the input. You'll study lifetimes properly in 0x07. Here, just notice that the compiler needs to be told. `slice.chunks(n)` panics when `n` is 0, so check first.

## Review it

Open [`review/README.md`](review/README.md). It shows five bugs from your own ALX C code, plus a small Rust pull request. For each bug, decide whether Rust would catch it at compile time, catch it at run time, or not catch it at all. Then compare with `review/ANSWERS.md`.

## Explain it

In your notes, write three or four sentences for a non-technical training manager: *"Why is Rust a safer choice than C for the system that decides whether a staff member is allowed to work?"* No jargon: no "borrow checker", no "memory safety". Talk about the kind of mistake it prevents.

## Check yourself

Answer each one before you open the answers. If you're not sure, try it in a scratch file.

1. **Will this compile?**

   ```rust
   fn main() {
       let name = String::from("Amina");
       let badge = name;
       println!("{name} {badge}");
   }
   ```

2. **Will this compile?**

   ```rust
   fn main() {
       let mut codes = vec![String::from("DG"), String::from("FIRE")];
       let first = &codes[0];
       codes.push(String::from("AVSEC"));
       println!("{first}");
   }
   ```

3. **Will this compile?**

   ```rust
   fn tidy(name: &str) -> &str {
       let tidy = name.trim().to_uppercase();
       &tidy
   }
   ```

4. **What prints?**

   ```rust
   fn main() {
       let line = String::from("  Amina Wanjiru ");
       let trimmed = line.trim();
       println!("{}", trimmed.as_ptr() == line[2..].as_ptr());
       println!("{} {}", line.len(), trimmed.len());
   }
   ```

5. **Will this compile?** It's the dedupe from task 3, written the obvious way.

   ```rust
   fn main() {
       let mut codes = vec!["DG", "FIRE", "DG"];
       codes.retain(|c| codes.iter().filter(|x| *x == c).count() == 1);
       println!("{codes:?}");
   }
   ```

6. **Why** does `badge_lines` in task 4 take `Vec<String>` instead of `&[String]`? What can the caller no longer do after calling it?

<details>
<summary>Answers</summary>

1. No: error `E0382`, borrow of moved value `name`. `let badge = name;` moved the `String`, so `name` is no longer usable. Borrow it (`let badge = &name;`) or clone it if you really need two copies.
2. No: error `E0502`, cannot borrow `codes` as mutable because it is also borrowed as immutable. `push` may move the whole list to a bigger block of memory, which would leave `first` pointing at freed memory. Print `first` before the `push`, or copy out what you need.
3. No: error `E0515`, cannot return reference to local variable `tidy`. The `String` is freed when the function ends, so the reference would dangle. Return the `String` itself: `fn tidy(name: &str) -> String`.
4. `true`, then `16 13`. `trim` returns a slice of the same memory, starting two bytes in: nothing is copied. The `String` is still 16 bytes long; the slice is 13.
5. No: error `E0502` again. `retain` borrows `codes` mutably for the whole call, and the closure tries to read `codes` at the same time. That's why task 3 keeps a separate `HashSet` of codes already seen.
6. The function owns the strings, so it can change them in place and hand them back without allocating new ones. The caller gave the `Vec` away: using it after the call is the "borrow of moved value" error from question 1. If the caller still needs it, it must pass a clone, which makes the cost visible.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs` with `pub mod on_your_own;` in `src/lib.rs`, and write your tests in `tests/on_your_own.rs`.

**The brief.** A small roster toolkit, one function for each kind of access:

- `pub fn surnames(names: &[String]) -> Vec<&str>`: the last word of each name, borrowed from the input
- `pub fn shout_all(names: &mut Vec<String>)`: every name in capitals, changed in place
- `pub fn split_off_first(names: Vec<String>, n: usize) -> (Vec<String>, Vec<String>)`: the first `n` names and the rest, handed back as owned lists

**Constraints**

- `surnames` doesn't allocate any text: no `clone`, `to_string`, `to_owned` or `format!`. Names that are empty or only spaces are left out.
- `shout_all` keeps the same `Vec`: the caller's list must still be the one they passed in.
- `split_off_first` never clones a `String`, and an `n` bigger than the list gives everything first and an empty rest.
- No `unwrap()`.

**Done when**

- A test proves each surname points into the input, by comparing `as_ptr()` like task 0's test does
- A test checks `split_off_first` with `n` of 0, the list's length, and more than its length
- `cargo test -p ownership-memory --test on_your_own` passes and clippy prints nothing
- You can say, for each of the three signatures, who owns the names after the call

## Checkpoint

- `cargo test -p ownership-memory` passes and clippy is clean
- Your review matches the answer key on at least four of the five ALX bugs
- You can explain, without notes, the difference between `String`, `&str` and `&mut String`

## When you're stuck

- **"borrow of moved value"**: the value was given away earlier. Borrow it (`&name`) instead of moving it, or move it later.
- **"cannot borrow `x` as mutable because it is also borrowed as immutable"**: a reader still exists while you try to write. Finish reading first, or copy out what you need before you change anything.
- **"missing lifetime specifier"**: you're returning a reference and the compiler can't tell which input it borrows from. Usually the fix is to make it clear there's only one input it could come from.
- **"returns a value referencing data owned by the current function"**: you're returning a borrow of something that's about to be freed. Return the owned value instead.
- **An error code you don't recognise?** [Common compiler errors](../docs/common-errors.md) has a tiny example and the usual fixes for each, including [E0382, E0499, E0502 and E0106](../docs/common-errors.md).
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
