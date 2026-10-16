# Answers

1. **`load_config` panics on every expected problem.** A missing file, an empty file, or a typo in the number all crash the app *at start-up*, at a station, possibly offline, with no way for staff to fix it. These are normal situations, so return `Result<StationConfig, ConfigError>` and let the caller decide (show a message, fall back to defaults, report it).

2. **`.expect("bad number")` loses the information that matters.** It doesn't say which file, which line or what the bad text was. A `ConfigError::BadAlertDays { path, value }` does.

3. **`save_config` throws its error away and then reports success.** `let _ = fs::write(...)` ignores the `Result`, and `Ok(())` tells the caller it worked. A full disk or a read-only folder silently loses the settings. Use `fs::write(...)?`. This one is the worst bug in the PR, because nobody will ever see it fail.

4. **`save_config` isn't safe against power cuts.** If the power drops mid-write, the file is half-written and `load_config` will then crash on the next start. Use the write-to-temp-then-rename pattern from task 3.

5. **`catch_unwind` used as error handling.** Catching a panic to get a default is a sign the function should have returned a `Result` in the first place. It also hides *which* problem happened, and some panics can't be caught at all (with `panic = "abort"` set for release builds, it does nothing). With a `Result`, it's one line: `load_config(path).map(|c| c.alert_days).unwrap_or(30)`. Even then, ask whether silently using 30 is right, or whether staff should be told their settings couldn't be read.

6. **`Box<dyn Error>` in a library API** means callers can't `match` on what went wrong. It's fine in a `main` function or a quick script. For the app's own modules, use a specific error enum.

7. **`path: &str`** is better as `&Path` (or `impl AsRef<Path>`): paths aren't always valid text on every operating system.

8. **"Works on my machine"** — ask for tests: a missing file, an empty file, a bad number, and save-then-load round-tripping. `tempfile::tempdir()` makes these easy.

Priority when you write the review: lead with #3 (silent data loss) and #1 (crash at start-up). Style points like #7 go last, or in a single "nits" comment.
