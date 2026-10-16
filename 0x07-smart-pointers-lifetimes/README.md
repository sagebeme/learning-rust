# 0x07. Smart pointers and lifetimes

**Week 3** · When one owner isn't enough, and how the compiler tracks borrows over time.

ALX's linked lists, revisited, and why Rust code usually doesn't build them. By the end of this project you've covered everything Rust does with memory that the TWA work needs.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x06 done | `cargo test -p collections-iterators` passes | Go back to [0x06](../0x06-collections-iterators) |
| Ownership and borrowing, solidly | You can explain why a function returning `&str` needs its input to live long enough | Re-read the Book's [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) |

## The story so far

You can handle 800 records. Some data doesn't fit the one-owner rule, though: a rule tree like "airside safety, and dangerous goods or its equivalent", a log several parts of the app write to, a nightly count across four stations at once, and the web of prerequisites behind Amina's forklift licence.

## What you'll be able to do

- Use `Box` for recursive data, `Rc`/`RefCell` for shared ownership in one thread, `Arc`/`Mutex` across threads
- Read and write lifetime annotations like `'a`, and say what they promise
- Model a graph (prerequisites) with indexes or keys instead of pointers

## Read first

- The Book: [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) (`Box`, `Rc`, `RefCell`), [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) (threads, `Mutex`, `Arc`), and [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- *Rust by Example*: [Box, stack and heap](https://doc.rust-lang.org/rust-by-example/std/box.html), [Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html), [Arc](https://doc.rust-lang.org/rust-by-example/std/arc.html), [Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/), chapter 1 only. It shows why linked lists are hard in Rust, and why you rarely need one.
- [Rustlings](https://rustlings.rust-lang.org/): `smart_pointers`, `lifetimes`, `threads`

**Go deeper:** [Rust Atomics and Locks](https://marabos.nl/atomics/) by Mara Bos, chapters 1 and 2 (free online), and [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) for threads that borrow instead of needing `Arc`.

## Which one, when

| You need | Use | Memory cost |
| --- | --- | --- |
| A value that contains itself (a tree) | `Box<T>` | one heap allocation |
| Several owners, one thread | `Rc<T>`, plus `RefCell<T>` if they must change it | a counter; `RefCell` checks borrows *at run time* |
| Several owners, several threads | `Arc<T>`, plus `Mutex<T>` (or `RwLock`) to change it | an atomic counter and a lock |
| A graph or list with links | a `Vec` or map, with indexes or keys as the links | none extra, and no reference cycles |

`RefCell` and `Mutex` move the borrow rules from compile time to run time: break them and you get a panic or a wait. Reach for them only when the plain `&`/`&mut` rules genuinely can't express what you need.

## Tasks

```bash
cargo test -p smart-pointers-lifetimes
```

### 0. Rules as a tree (`t00_rule_tree.rs`)

`Rule::All` contains two more `Rule`s. A type can't contain itself directly (its size would be infinite), so the children are `Box<Rule>`: a pointer, with a fixed size. `match` on `self` and call `is_met` recursively. A `Box` derefs to what it holds, so `left.is_met(held)` just works.

This is a small version of how the TWA engine will express "requires FIRE and either DG or AVSEC".

### 1. A borrowing index (`t01_name_index.rs`)

`NameIndex<'a>` holds `&'a str` slices of some text. The `'a` says: *this index can't outlive the text it points into*. The key detail is the return type of `starting_with`: `Vec<&'a str>`, not `Vec<&str>`. The plain version would tie the results to the *index* (`&self`). With `'a`, they're tied to the *text*, so they stay valid after the index is dropped. One of the tests checks exactly that.

### 2. A shared log (`t02_alert_log.rs`)

`Rc::clone(&rc)` makes a new handle to the *same* data and bumps a counter; when the last handle is dropped, the data is freed. That's reference counting, automatic. `Rc` only gives you shared (read-only) access, so the log is wrapped in `RefCell`, which lets you `borrow_mut()` through a `&self`. `Rc::strong_count` tells you how many handles exist.

### 3. Threads with a shared total (`t03_parallel_tally.rs`)

`Rc` isn't safe across threads, and the compiler won't let you try. Use `Arc` (atomic `Rc`) and a `Mutex`:

1. `let total = Arc::new(Mutex::new(0));`
2. for each station: `let total = Arc::clone(&total);` then `thread::spawn(move || { ... })`
3. inside the thread, count *first*, then lock and add. Hold a lock for as short a time as you can.
4. `join()` every thread before reading the total

`lock()` returns a `Result`, because a thread that panicked while holding the lock "poisons" it. The solution shows one reasonable way to handle that without `unwrap`.

### 100. Prerequisites as a graph (`t100_prerequisite_graph.rs`) — advanced

In C you'd link nodes with pointers. In Rust, store the graph in a map: qualification → the set of qualifications it needs. To find a training order, keep taking any qualification whose prerequisites are all done (Kahn's algorithm). If you get stuck with qualifications left over, there's a cycle. Using `BTreeMap` and `BTreeSet` makes "alphabetical among the ready ones" automatic.

The TWA engine needs exactly this to reject a catalogue where DG requires FIRE and FIRE requires DG.

## Review it

Open [`review/README.md`](review/README.md): a PR that reaches for `clone`, `Rc<RefCell>` and `Mutex` where it doesn't need them.

## Explain it

The training manager wants to add a rule: "Ramp supervisor requires Ramp, and Ramp requires Ramp supervisor." Explain in two sentences, without technical words, why the system refuses it and what they probably meant instead.

## Checkpoint

- `cargo test -p smart-pointers-lifetimes` passes and clippy is clean
- **End of the memory track.** In your notes, without looking anything up, explain to yourself when memory is freed for: a `String` in a local variable, a `Box`, an `Rc` with three handles, and a value moved into a thread.

## When you're stuck

- **"recursive type has infinite size"**: put the recursive part in a `Box`.
- **"`Rc<...>` cannot be sent between threads safely"**: use `Arc` for anything shared with a thread.
- **"already borrowed: BorrowMutError"** panic: you called `borrow_mut()` while another borrow was alive. Make the borrows shorter.
- **"lifetime may not live long enough"** or "borrowed value does not live long enough": something you return or keep points at a value that's freed first. Check which input the `'a` connects to.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
