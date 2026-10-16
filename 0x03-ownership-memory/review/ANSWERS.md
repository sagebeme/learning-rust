# Answers

## Part 1

**A. strdup without a terminator.** The copy is never given its closing `'\0'`, so anything that reads it runs past the end into whatever memory follows. *Rust: impossible by design.* A `String` stores its length; there is no terminator to forget. `name.to_string()` or `.clone()` is the whole function.

**B. Freeing `grid[height]`.** The valid rows are `0..height`, so the first `free` is one past the end: undefined behaviour, often a crash later and somewhere else. *Rust: not even written.* You never free a `Vec<Vec<i32>>` by hand; dropping it frees every row. If you did index `grid[height]`, that's a **run-time panic** with "index out of bounds", immediately and at the right line, instead of silent corruption.

**C. realloc copying `old_size` bytes into a smaller block.** It writes past the end of `new_ptr`: a heap overflow, the classic security bug. *Rust: run time.* `Vec::resize` and `truncate` do this safely. A hand-written copy with slices, `new[..old.len()].copy_from_slice(&old)`, **panics** when the sizes don't match instead of overflowing.

**D. Deleting at index == length.** The bounds check uses `>` instead of `>=`, so it walks to the end and follows a NULL pointer. *Rust: compile time, mostly.* There's no NULL. A missing node is `Option::None`, and you can't read `.next` from a `None` without handling that case first. `Vec::remove(i)` with `i == len` **panics** with a clear message.

**E. Not counting nodes whose value is 0.** `h->n != '\0'` compares the number to 0, so a list `[1, 0, 2]` reports length 2. *Rust: not at all.* It's a logic bug. Rust would happily compile `if node.n != 0 { count += 1 }`. The only defences are a test with a zero in it, and a reviewer who asks why that line exists. That's why you're learning to review.

The pattern: Rust removes almost every *memory* bug (A–D), mostly before the code runs. It does nothing about *logic* bugs (E). Reviews and tests are still your job.

## Part 2: review comments

1. **`names: Vec<String>` takes ownership it doesn't need.** The function only reads the names, so take `&[String]`. Then callers don't have to `staff.clone()` the whole list just to call it.
2. **`station: &String` should be `&str`.** `&str` accepts `&String`, string literals and slices. `&String` forces callers to allocate, as in `&"NBO".to_string()`. Clippy warns about this one (`ptr_arg`).
3. **`names.clone()` in the loop** copies every string again, for nothing. Iterate with `for name in names` (over `&[String]`).
4. **`let n = name.clone();`** is a third copy of each name, and **`station.clone()`** inside `format!` is pointless: `format!` only needs to read it.
5. **Suggested version:**

   ```rust
   pub fn station_badges(names: &[String], station: &str) -> Vec<String> {
       names
           .iter()
           .map(|name| format!("{} - {station}", name.to_uppercase()))
           .collect()
   }
   ```

A good review comment says *what*, *why* and *what instead*, like each point above. "Too many clones" on its own teaches nobody anything.
