# Answers

1. **Three booleans allow eight states, and only three or four are real.** Nothing stops `is_valid: true, is_expired: true`. What does `can_work` do with that? It says yes. Replace the flags and the "ignore this if…" fields with one enum, like `QualificationStatus` from task 1, so only real states can be built.

2. **Comments that say "ignore this if…" are a warning sign.** `days_left` and `grace_days_used` are only meaningful in some states, so they belong *inside* those enum variants (`Valid { days_left }`, `InGrace { days_overdue }`). Then you can't read them in the wrong state.

3. **The grace rule is hard-coded in `can_work`.** `< 14` is a business rule (and is it `<` or `<=`?), buried in an `if`. It belongs on the qualification type, as `grace_days` from task 3, so each qualification can have its own and nobody has to grep for 14.

4. **`station: String` accepts any text.** `"nbo"`, `"Nairobi"` and `""` are all accepted, and `station_name` quietly calls all of them "Eldoret". That's a wrong answer delivered with confidence, which is worse than an error. Use the `Station` enum: parse once, at the edge where data comes in, then every other function receives a value that can only be one of the four.

5. **`staff_number: String`** allows letters and blanks. Use a number type, or a small `StaffNumber` type that validates in its constructor.

6. **"Tested by hand"** isn't enough for code that decides whether someone may work airside. Ask for unit tests for each state, especially the boundaries: day 0, the last grace day, the first day after it.

Suggested shape:

```rust
pub struct QualificationRecord {
    pub staff_number: u32,
    pub station: Station,
    pub qualification: String,
    pub status: QualificationStatus,
}

pub fn can_work(record: &QualificationRecord) -> bool {
    record.status.is_usable()
}
```

How to word it in the PR: lead with the impact ("a record can be valid and expired at the same time, and `can_work` says yes"), then the fix. Reviewers who explain consequences get listened to.
