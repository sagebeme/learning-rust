# 0x06. Collections and iterators

**Week 3** · Lists, maps and sets, and the iterator style that makes Rust code short and hard to get wrong.

ALX's hash tables and search algorithms, revisited. You'll rebuild your hash table, fix the bug it had, and write a binary search that can't run off the end.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| 0x05 done | `cargo test -p errors-files-cli` passes | Go back to [0x05](../0x05-errors-files-cli) |
| To know what a closure is, roughly | You can read `\|x\| x + 1` as "a small function that adds one" | The Book's [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html) section |

## The story so far

The roster loads safely. Now there are 800 records, not one. The training manager wants counts per station and a list of what's expiring this month. Amina is one row among many, and the answers must come out in the same order every time.

## What you'll be able to do

- Pick the right collection: `Vec`, `HashMap`, `BTreeMap`, `HashSet`
- Chain `filter`, `map`, `sum`, `collect` instead of writing index loops
- Explain why output order matters for the TWA engine, and get it right every time

## Read first

- The Book: [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html), [Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- *Rust by Example*: [Iterator::any](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_any.html), [Iterator::find](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples/iter_find.html), [HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- [Rustlings](https://rustlings.rust-lang.org/): `hashmaps`, `iterators`

**Go deeper:** [`std::collections`](https://doc.rust-lang.org/std/collections/index.html) (its first page explains when to use which collection), and the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait's list of methods.

## `HashMap` or `BTreeMap`?

A `HashMap` is fast, but its iteration order is **random, and changes between runs**. A `BTreeMap` keeps keys sorted. The TWA engine must give *the same answer, in the same order, every time*, on every device, so anything that ends up in output, a report or a test uses `BTreeMap` (or a sorted `Vec`). Use `HashMap` only for lookups whose order nobody sees.

## Tasks

```bash
cargo test -p collections-iterators
```

`src/model.rs` gives you `Station` and `Holding`, one person's record for one qualification.

### 0. Count by station (`t00_count_by_station.rs`)

The *entry API* is the idiom for counting: `*counts.entry(key).or_insert(0) += 1;`. It finds the slot or creates it, in one lookup.

### 1. What expires soon? (`t01_expiring_within.rs`)

`.iter().filter(...).collect()` to a `Vec<&Holding>`, then sort. `sort_by_key(|h| (h.days_left, h.staff_number))` sorts by the first value, then the second. Tuples compare in order. `(0..=days).contains(&x)` reads like the rule.

### 2. Your hash table, rebuilt (`t02_chained_map.rs`)

Buckets are a `Vec<Vec<(String, String)>>`. To insert:

1. work out the bucket with `djb2(key) % buckets.len()`
2. walk **that bucket's chain** looking for the key, and replace the value if you find it (`std::mem::replace` hands back the old value)
3. otherwise push a new pair onto the chain

Your ALX `hash_table_set` walked to the *next bucket* instead of along the chain. A test here forces every key into one bucket, so that bug can't hide.

### 3. Binary search without the ALX bug (`t03_find_staff.rs`)

Your ALX version used `right = middle - 1` with unsigned numbers. When the target was smaller than the first element, `right` went below zero, wrapped around to an enormous number, and the search read far past the array. In Rust, that subtraction would *panic* in a debug build, so it's caught, but it's still a crash.

The robust shape searches the half-open range `low..high`, where `high` is one *past* the end, so nothing ever subtracts below zero:

- the loop runs while `low < high`
- too small: `low = middle + 1`
- too big: `high = middle`

### 100. Who holds each qualification? (`t100_group_by_code.rs`) — advanced

`entry(...).or_default().push(...)`, then `sort_unstable()` and `dedup()` each list. The keys are `&str` borrowed from the holdings: no allocation for the keys at all.

## Review it

Open [`review/README.md`](review/README.md): your ALX hash-table bug, translated into Rust, plus a report that comes out in a different order every run.

## Explain it

A station manager runs the same "expiring this month" report twice and the names come out in a different order. They ask if the system is broken. Write your answer in two or three sentences: what was happening, and what you changed so it can't happen again.

## Check yourself

Answer each one before you open the answers. If you're not sure, try it in a scratch file.

1. **What prints?** And what would change if it were a `HashMap`?

   ```rust
   use std::collections::BTreeMap;

   fn main() {
       let mut counts = BTreeMap::new();
       for station in ["NBO", "MBA", "NBO", "KIS", "NBO"] {
           *counts.entry(station).or_insert(0) += 1;
       }
       println!("{counts:?}");
   }
   ```

2. **What prints, and in what order?**

   ```rust
   fn main() {
       let hours = [8, 9, 7];
       let doubled = hours.iter().map(|h| {
           println!("doubling {h}");
           h * 2
       });
       println!("before");
       let total: i32 = doubled.sum();
       println!("{total}");
   }
   ```

3. **Will this compile?**

   ```rust
   fn main() {
       let hours = [8, 9, 7];
       let total = hours.iter().sum();
       println!("{total}");
   }
   ```

4. **Spot the bug.** `find(&[10, 20, 30], 20)` returns `Some(1)`. What happens with `find(&[10, 20, 30], 5)`, and with an empty slice?

   ```rust
   fn find(sorted: &[u32], target: u32) -> Option<usize> {
       let mut left = 0;
       let mut right = sorted.len() - 1;
       while left <= right {
           let middle = (left + right) / 2;
           if sorted[middle] == target {
               return Some(middle);
           }
           if sorted[middle] < target {
               left = middle + 1;
           } else {
               right = middle - 1;
           }
       }
       None
   }
   ```

5. **Why** is `*counts.entry(key).or_insert(0) += 1` better than checking `contains_key` and then calling `get_mut` or `insert`?

<details>
<summary>Answers</summary>

1. `{"KIS": 1, "MBA": 1, "NBO": 3}`. A `BTreeMap` keeps its keys sorted, so the output is the same every run. A `HashMap` would give the same counts in an order that can change from one run to the next, which is the bug in this project's review.
2. `before`, then `doubling 8`, `doubling 9`, `doubling 7`, then `48`. Iterators are lazy: `map` only describes the work, and nothing runs until something like `sum` or `collect` asks for the values.
3. No: error `E0283`, type annotations needed. `sum` can produce several types, and nothing says which one you want. Write `let total: i32 = ...` or `.sum::<i32>()`.
4. With 5, the first `middle` is 0, `10 > 5`, and `right = 0 - 1` panics with "attempt to subtract with overflow" (in a release build it would wrap to a huge number). An empty slice panics even earlier, on `sorted.len() - 1`. That's your ALX bug. Searching `low..high` with `high = middle` never subtracts, as task 3 shows.
5. It's one lookup instead of two, it reads as one idea ("find the slot or make it, then add one"), and it avoids fighting the borrow checker over holding a reference from `get_mut` while deciding to `insert`.

</details>

## On your own

No steps, no tests given, no solution. Put it in this project as `src/on_your_own.rs` with `pub mod on_your_own;` in `src/lib.rs`, and use `Holding` and `Station` from `src/model.rs`.

**The brief.** Each station needs a minimum number of people holding certain qualifications: at least 3 with `DG`, at least 2 with `FIRE`, and so on. Write `pub fn coverage_gaps(holdings: &[Holding], needed: &[(&str, usize)]) -> BTreeMap<Station, Vec<(String, usize)>>`: for each station that's short, which codes are short and by how many.

**Constraints**

- Only holdings that are still valid count (`days_left` of 0 or more).
- One person holding the same code twice counts once.
- Stations with no gaps don't appear. Each station's list is sorted by code.
- A station with no holdings at all is short of everything in `needed`, so check all four stations, not just the ones in the input.
- No index loops (`for i in 0..`): iterators and the entry API. The same input must give the same output every run, in any order.

**Done when**

- Tests cover an expired holding, a duplicate holding, a station that's fine, and a station with nobody
- One test reverses the input and checks the result is identical
- `cargo test -p collections-iterators --test on_your_own` passes and clippy prints nothing

## Checkpoint

- `cargo test -p collections-iterators` passes and clippy is clean
- You can say, for each of `Vec`, `HashMap`, `BTreeMap` and `HashSet`, one place in TWA you'd use it

## When you're stuck

- **"cannot borrow `self.buckets` as mutable more than once"**: take one `&mut` to the chain (`let chain = &mut self.buckets[index];`) and work with that.
- **`attempt to subtract with overflow`** in the binary search: you've rebuilt the ALX bug. Use the half-open range.
- **A `collect()` that won't compile**: tell it what to build, with `let v: Vec<_> = ...collect();`
- **An error code you don't recognise?** [Common compiler errors](../docs/common-errors.md) has a tiny example and the usual fixes for each, including [E0499](../docs/common-errors.md#e0499-cannot-borrow-as-mutable-more-than-once-at-a-time).
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
