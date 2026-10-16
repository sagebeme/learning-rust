# 0x03. Ownership and memory

**Week 2** · The idea the rest of Rust is built on: every value has exactly one owner.

In C (your ALX projects), *you* decided when memory was allocated and freed, and the compiler trusted you. Every `malloc` needed a matching `free`, at the right moment, exactly once. Rust makes the same decisions, but the compiler checks them before the program ever runs.

## What you'll be able to do

- Say who owns a value at any point in a function, and when it gets freed
- Choose between taking ownership (`String`), borrowing to read (`&str`, `&[T]`) and borrowing to change (`&mut`)
- Return references and slices that point into the caller's data, without copying it

## Read first

- The Book: **Understanding Ownership** (all three sections). Read it twice; it's that important.
- *Rust by Example*: **Scoping rules** (RAII, ownership and moves, borrowing)
- Rustlings: `move_semantics`, `strings`, `vecs`

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

## Checkpoint

- `cargo test -p ownership-memory` passes and clippy is clean
- Your review matches the answer key on at least four of the five ALX bugs
- You can explain, without notes, the difference between `String`, `&str` and `&mut String`

## When you're stuck

- **"borrow of moved value"**: the value was given away earlier. Borrow it (`&name`) instead of moving it, or move it later.
- **"cannot borrow `x` as mutable because it is also borrowed as immutable"**: a reader still exists while you try to write. Finish reading first, or copy out what you need before you change anything.
- **"missing lifetime specifier"**: you're returning a reference and the compiler can't tell which input it borrows from. Usually the fix is to make it clear there's only one input it could come from.
- **"returns a value referencing data owned by the current function"**: you're returning a borrow of something that's about to be freed. Return the owned value instead.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
