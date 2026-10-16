# Answers

## `NameCache::set`

1. **It walks neighbouring buckets instead of the chain.** This is your ALX `hash_table_set` bug, in Rust. The loop moves to `index + 1`, the next *bucket*, but a person who collided is further along the *same* bucket's list. The result: setting the same staff number twice can add a duplicate instead of updating. Rust compiles it without complaint, because it's a logic bug.

2. **It only ever checks the first entry of each bucket** (`[index][0]`). Even within the right bucket, anyone after the first is never found.

3. **It can stop early, or check the wrong person.** The loop stops at the first empty bucket, so it gives up on keys it could have found. And it compares against people who live in *other* buckets, so a different person's entry could match by accident if this code ever changes.

4. **`self.buckets.len()` can be 0**, and `% 0` panics. There's no constructor here to guarantee at least one bucket.

5. **Do you need this at all?** `HashMap<u32, String>` does all of this correctly, faster, in one line. A hand-built hash table in production code needs a very good reason. Ask for it in the review.

Suggested fix, if a cache really is needed:

```rust
pub fn set(&mut self, staff_number: u32, name: String) {
    self.names.insert(staff_number, name); // names: HashMap<u32, String>
}
```

## `station_report`

6. **The order changes every run.** `HashMap` iteration order is randomised per process, so the same data gives `NBO` first one run and `KIS` first the next. Every test that compares the whole report as a string will be flaky, and the training manager will think the numbers changed. Use `BTreeMap`, or sort before printing.

7. **Stations as `String`** again. `"NBO"`, `"nbo"` and `"NBO "` are counted as three stations. Use the `Station` enum.

8. **`format!` inside `push_str`** allocates a temporary string for every line. `use std::fmt::Write;` and `writeln!(report, "{station}: {count}")` writes straight into `report`. That's a nit, not a blocker: say so in the comment.

## About "all tests pass"

The tests clearly never set the same staff number twice with a collision, and never compare a report from a different run. When a PR says "all tests pass", a good review asks *which* cases the tests cover. Suggest the missing ones: a collision, an update after a collision, and a two-station report compared exactly.
