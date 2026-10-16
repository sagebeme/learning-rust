# Answers

1. **The lock is held for the whole loop, including the SMS call.** Each thread takes the lock *before* the loop and keeps it until the end, so the four "parallel" threads run one after another. The PR is no faster than the original, just more complicated. Worse, each thread holds the lock during a one-second network call, multiplied by every expired person. Count or collect locally first, then lock once to merge, or better, have each thread *return* its list and combine the results after `join()`. Then you need no `Mutex` at all:

   ```rust
   let handles: Vec<_> = stations
       .into_iter()
       .map(|station| thread::spawn(move || {
           station.into_iter().filter(|&(_, days)| days < 0).map(|(staff, _)| staff).collect::<Vec<_>>()
       }))
       .collect();
   ```

2. **Sending SMS from inside the scan** mixes two jobs. If the SMS provider is slow or down, the scan is slow or fails. Have the scan *find* expired people, and let a separate step send alerts (with retries, 0x0D). It also makes the scan testable without sending real messages.

3. **`Rc<RefCell<Vec<String>>>` for a catalogue that's only read.** Nothing here ever changes the catalogue, so `RefCell` adds run-time borrow checks for nothing, and `Rc` makes `Catalogue` impossible to share with the threads above. If it's shared read-only across threads, use `Arc<Vec<String>>` or just `&[String]`. If it's looked up often, use a `HashSet<String>`.

4. **`contains` clones the entire catalogue on every call**, then allocates a `String` for the code. `self.codes.borrow().iter().any(|c| c == code)` does neither. Called for every staff member, the original is quadratic and allocation-heavy.

5. **`station.clone()` inside the thread** copies the station's data that the thread already owns (it was moved in). Iterate it directly.

6. **`unwrap()` on `lock()` and `join()`**: if any thread panics, the scan panics too, and the error message won't say which station or why. At minimum use `expect` with a message. Better, return a `Result` from the scan.

7. **`let count = ...; count`** is a clippy `let_and_return` nit. Return the expression directly.

The lesson: smart pointers and locks are tools for specific problems. Every `Rc`, `RefCell`, `Arc` and `Mutex` in a PR should have a reason you could say out loud. If you can't name it, ask for the plain version.
