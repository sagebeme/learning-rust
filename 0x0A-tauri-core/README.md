# 0x0A. Tauri core

**Week 5** · Putting the engine inside the app: the Rust half of a Tauri app, and the boundary with the screens.

In week 1 you called one Rust function from a Tauri screen. This week you build the Rust core properly: commands the screens can call, shared app state, errors the screens can show, data in the shape the screens expect, and the permission file that says which commands each window may call.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| Week 4 done, ideally | `cargo test -p rules-engine` passes | Not blocking: this project uses the reference engine until you switch (see below) |
| Your week 1 Tauri spike (for the checkpoint) | It runs on your desktop | Redo [the week 1 spike](../docs/week-1-tauri.md); you need [Node.js](https://nodejs.org/) and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) |
| To know what JSON looks like | You can read `{"staffNumber": 48213}` | Two minutes on [json.org](https://www.json.org/json-en.html) |

## The story so far

The engine answers Amina's question in a test. Now she, or the training officer at her station, needs to see that answer in the TWA app. This week you build the Rust half of the app: what the screens can ask for, and how the answer gets back to them safely.

### Using your own engine

This project depends on the engine. By default it uses the reference engine in `engine-reference/`, so it works even if your 0x09 isn't finished. Once your 0x09 passes, open this project's `Cargo.toml` and swap the `twa-engine` line for the one in the comment above it. Run the tests again: if they still pass, your engine and the reference agree.

## What you'll be able to do

- Write Rust functions that become Tauri commands, and keep them testable without starting the app
- Share state safely between commands, and turn every failure into an error the screens can display
- Convert between Rust's `snake_case` and the screens' `camelCase` JSON without writing it by hand
- Review a Tauri permissions file the way you'd review code

## Read first

- Tauri 2 docs: [Calling Rust from the Frontend](https://v2.tauri.app/develop/calling-rust/) (commands, arguments, errors), [State Management](https://v2.tauri.app/develop/state-management/), [Capabilities](https://v2.tauri.app/security/capabilities/), and [Calling the Frontend from Rust](https://v2.tauri.app/develop/calling-frontend/) (events)
- serde docs: [Container attributes](https://serde.rs/container-attrs.html) (`rename_all`, `deny_unknown_fields`) and [Implementing Serialize](https://serde.rs/impl-serialize.html)

**Go deeper:** Tauri's [security overview](https://v2.tauri.app/security/), and the [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) docs on poisoning.

## How this project fits Tauri

A Tauri command is a normal Rust function with an attribute:

```rust
#[tauri::command]
fn add_holding(
    state: tauri::State<'_, AppState>,
    input: NewHolding,
) -> Result<StaffSummary, CommandError> {
    tauri_core::t02_app_state::add_holding(&state, input)
}
```

Building a Tauri app needs each platform's web view libraries, which makes CI slow and fragile. So this project keeps all the real logic in **plain functions** that take `&AppState`, and the `#[tauri::command]` wrappers stay one line long. `tauri::State<'_, AppState>` derefs to `&AppState`, so the wrapper just passes it through. The tests here run the logic directly; your week 1 spike shows the wiring.

Registering everything in the app's `src-tauri/src/lib.rs`:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![add_holding, staff_summary])
        .run(tauri::generate_context!())
        .expect("error while running the TWA app");
}
```

## Tasks

```bash
cargo test -p tauri-core
```

### 0. Errors the screens can show (`t00_command_error.rs`)

When a command returns `Err`, Tauri serializes the error and the screen's `invoke(...)` promise rejects with it. So the error type must implement `serde::Serialize`. Give the screens something useful, `{ "kind": "notFound", "message": "No staff member 48213" }`: a stable `kind` the screen can switch on, and a `message` a person can read.

Implement `kind()`, then implement `Serialize` by hand: build a small `ErrorPayload` struct and serialize *that*. This is the pattern the Tauri docs recommend.

### 1. Input from the screens (`t01_inputs.rs`)

The screens send JSON with `camelCase` keys (`staffNumber`); Rust uses `snake_case` (`staff_number`). `#[serde(rename_all = "camelCase")]` converts both ways. `deny_unknown_fields` turns a typo like `staffNumbr` into an error instead of a silently missing value.

Then validate what serde can't: an empty code, a completion date in the future. **Never trust the screens.** In a Tauri app, the screen code runs in a web view, and anything it sends should be checked as if it came from the internet.

### 2. State shared by commands (`t02_app_state.rs`)

Tauri may run commands at the same time on different threads, so shared state needs a `Mutex`. Each command locks, does its work, and lets go, quickly. A poisoned lock becomes `CommandError::Busy` rather than a panic: a panic in a command would take the whole app down.

### 3. Permissions (`t03_capabilities.rs`)

In Tauri 2, a window can only call commands its **capabilities** file allows. Write a checker for a simplified version of that file, modelled on Tauri's format. Every registered command needs an `allow-<command-name>` permission (with the name in kebab-case), and every `allow-` permission must match a real command. A permission for a command that no longer exists is how a deleted feature can stay reachable.

### 4. The gap check as a command (`t04_gap_for.rs`)

The moment the app meets your engine. `gap_for` takes a person, a department, a role and a date, copies their holdings out of the state, releases the lock, and asks `twa_engine::t04_gap_check::gap_check`. The engine's errors become command errors: an unknown role is something the person can fix (`invalid`), and a broken catalogue isn't (`internal`). The tests ask Amina's question: on 19 October 2026 she isn't ready for Cargo, because her forklift licence is missing and her dangerous goods certificate expires in 13 days.

In the app, the screen calls it with `invoke("gap_for", { staffNumber: 48213, department: "Cargo", role: "Handler", today: "2026-10-19" })`.

### 100. Events (`t100_events.rs`) — advanced

Commands are the screens asking Rust. **Events** are Rust telling the screens, for example when the daily check finds an expiring licence. Build the payload struct and the event name. In the app, `app.emit("qualification-expiring", payload)` sends it, and the screen listens with `listen("qualification-expiring", ...)`.

## Review it

Open [`review/README.md`](review/README.md): a set of Tauri commands with a secret in the app, a panic in a command and a wide-open permissions file.

## Explain it

The client's IT manager asks: *"If the app runs on staff phones, can someone tamper with it to mark themselves as qualified?"* Write your answer in four or five sentences: what the app is allowed to decide on its own, what only the server decides, and why.

## Checkpoint

- `cargo test -p tauri-core` passes and clippy is clean
- Move your week 1 spike onto this code: wrap `add_holding`, `staff_summary` and `gap_for` as commands, show Amina's gap report on a screen, and show a `notFound` error nicely. It runs on desktop and Android.
- You've switched to your own engine, and the tests still pass

## When you're stuck

- **The screen gets `undefined` for a field**: the names don't match. Check `rename_all = "camelCase"` on both the input and output structs.
- **"the trait `Serialize` is not implemented for `CommandError`"**: task 0. Tauri requires it for any error a command returns.
- **A command works on desktop but the screen gets "not allowed"**: the capabilities file doesn't grant it, which is task 3's lesson.
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
