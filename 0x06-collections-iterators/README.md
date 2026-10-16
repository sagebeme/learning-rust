# 0x06. Collections and iterators

**Week 3** · Lists, maps and sets, and the iterator style that makes Rust code short and hard to get wrong.

ALX's hash tables and search algorithms, revisited. You'll rebuild your hash table, fix the bug it had, and write a binary search that can't run off the end.

## What you'll be able to do

- Pick the right collection: `Vec`, `HashMap`, `BTreeMap`, `HashSet`
- Chain `filter`, `map`, `sum`, `collect` instead of writing index loops
- Explain why output order matters for the TWA engine, and get it right every time

## Read first

- The Book: **Common Collections**, **Processing a Series of Items with Iterators**
- *Rust by Example*: **Iterator::any**, **Iterator::find**, **HashMap**
- Rustlings: `hashmaps`, `iterators`

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

## Checkpoint

- `cargo test -p collections-iterators` passes and clippy is clean
- You can say, for each of `Vec`, `HashMap`, `BTreeMap` and `HashSet`, one place in TWA you'd use it

## When you're stuck

- **"cannot borrow `self.buckets` as mutable more than once"**: take one `&mut` to the chain (`let chain = &mut self.buckets[index];`) and work with that.
- **`attempt to subtract with overflow`** in the binary search: you've rebuilt the ALX bug. Use the half-open range.
- **A `collect()` that won't compile**: tell it what to build, with `let v: Vec<_> = ...collect();`
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
