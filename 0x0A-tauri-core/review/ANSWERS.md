# Answers

1. **An API key compiled into the app.** Anything in the app can be pulled out of it: the binary, the installer, a rooted phone. Whoever extracts it can send SMS on the company's account. The key must live only on the server. The app asks the server to send a message, and the server checks who's asking. Rotate this key now, because it's in the git history too.

2. **`mark_qualified` lets the app decide who is qualified.** The client app can't be trusted with that decision: it runs on devices the company doesn't fully control. Recording a qualification must go through the server, with authentication, an audit trail and (probably) a second person approving. The app can *display* the engine's answer and work offline from synced data, but it must not be the authority.

3. **Panics in commands.** `lock().unwrap()` and `find(...).unwrap()` crash the whole app the moment someone looks up a staff number that doesn't exist. Return `Result<Staff, CommandError>`, with `NotFound` for a missing person and `Busy` for a poisoned lock (task 0 and task 2).

4. **`mark_qualified` silently does nothing** for an unknown staff number, and accepts any `code`, including empty, lower-case, or one that isn't in the catalogue. Validate the input and return an error the screen can show.

5. **`Result<(), String>`** throws away the structure of the error. The screen can't tell "invalid phone number" from "no network" except by parsing text. Use the serializable `CommandError` with a `kind`.

6. **The capabilities file grants far too much.**
   - `"windows": ["*"]` applies it to every window, including any added later.
   - `shell:allow-execute` lets screen code run programs on the device. If any page in the app ever shows untrusted content, that's remote code execution.
   - `fs:default` gives file-system access nobody asked for.

   Grant each window only the permissions it needs. "So nothing gets blocked" is the wrong goal for a permissions file: blocking is its job.

7. **`get_staff` clones the whole record** under the lock. That's fine at this size, but a summary type (task 2's `StaffSummary`) sends the screen only what it shows, which matters on slow devices.

Blockers first in the review: 1 (secret), 2 (authority), 6 (permissions), then 3–5. Points 1 and 2 are security issues: flag them to the lead as well as in the PR.
