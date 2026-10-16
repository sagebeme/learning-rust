# 0001. Client app technology

- **Status:** accepted
- **Date:**
- **Decided by:**

## Context

<!-- What problem does this decide? The app must run on Android, iOS, Windows, macOS and Linux,
     share one Rust rules engine, work offline at stations, and ship within the 4-month project. -->

## Options considered

The team compared Swift, Kotlin Multiplatform with Compose Multiplatform, Flutter, Tauri 2 and a pure-Rust UI (egui).

<!-- One line per option: what ruled it out, or what it would have cost. -->

## Decision

We will build the client app with **Tauri 2**: a Rust core that holds the rules engine and local data, with screens written in web technology.

<!-- Add one or two sentences on why, in your own words. -->

## Consequences

<!-- What this makes easier, what it makes harder, and what you'll have to watch. Cover at least:
     the four different web views, mobile plugin support, iOS builds needing a Mac, Tauri's
     permission (capabilities) model, and keeping secrets off the device. Add your spike results:
     build times, workarounds and app size per platform. -->

## How to explain it to the client

<!-- Two or three plain sentences for a non-technical manager. -->
