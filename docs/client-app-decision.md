# Week 1 decision: what the TWA client app is built with

**Owner:** you, as the architect · **Due:** end of week 1 · **Output:** `docs/decisions/0001-client-app.md`, filled in

The client app has to run on Android, iOS, Windows, macOS and Linux. The team is choosing between Kotlin, Swift and Flutter, and the Rust work raises two more options. This choice can't wait until you've finished learning Rust. Everything else gets built around it, so make it in week 1, with evidence.

The good news: **the rules engine is Rust whatever you pick.** The decision changes the outer layer and the "bridge" between the app and the engine, not the engine itself.

## The candidates

These are starting points, not conclusions. Confirm anything marked *check* in your spike (below), because these tools change quickly.

| Option | Android | iOS | Windows / macOS | Linux | How it calls the Rust engine |
| --- | --- | --- | --- | --- | --- |
| **Swift (SwiftUI)** | No | Yes | macOS only | No | UniFFI generates Swift bindings |
| **Kotlin Multiplatform + Compose Multiplatform** | Yes | Yes (*check* current status) | Yes | Yes | UniFFI generates Kotlin bindings. Using it from Kotlin code on iOS needs extra tooling (*check* Gobley, or a Swift layer) |
| **Flutter** | Yes | Yes | Yes | Yes | flutter_rust_bridge |
| **Tauri 2** (web UI inside a native shell, Rust core) | Yes | Yes | Yes | Yes | None needed: the core *is* Rust. The UI is web code, so the React skills from the admin app carry over |
| **Pure Rust UI (egui)**, the approach storytold's Craft apps use | *check* | *check* | Yes | Yes | None needed: everything is Rust |

Swift alone can't meet the requirement: no Android, Windows or Linux. Keep it only as half of a "Swift for Apple, Kotlin for the rest" plan, and price that as building the UI twice.

## What to weigh

Score each remaining option 1–5 on each line, and write one sentence of evidence per score.

1. **Every platform from one codebase**, including Linux, with no "community port" holding it together.
2. **The bridge to the Rust engine.** How much glue code? Who maintains it? Does it run in CI?
3. **Offline-first.** Station Wi-Fi drops. Can the app store data locally and sync later without fighting the framework?
4. **Team skills and hiring in Nairobi.** Who on the team already knows it? How hard is it to hire for in Nairobi?
5. **Accessibility and the look of a native app.** Screen readers, text scaling, platform conventions.
6. **Delivery risk in 4 months.** Maturity, docs, how often it breaks between versions.
7. **App store and enterprise distribution.** Signing, MDM for company devices, desktop installers.

## The spike: decide with evidence, not opinion

Pick your **two** best-scoring options and give each one day:

1. A screen that shows a list of five fake staff and their licence expiry dates.
2. One button that calls a tiny Rust function (for example the `expiry_label` you write in `0x01`) through that option's bridge, and shows the result.
3. Build it for at least Android and Linux. If you have a Mac, add iOS.
4. Write down: time to first working build, anything that needed a workaround, and app size.

The spike only has to prove the bridge works and the build is sane. It's throwaway code.

## Then write it down

Fill in `docs/decisions/0001-client-app.md`. A decision record is short: the context, the options, the decision, and what it costs you. In a client meeting, it's also what you point to when someone asks "why didn't you just use X?"

## Checkpoint

- The decision record is filled in, with scores and spike results
- You can explain the choice to a non-technical manager in under two minutes, without naming a single framework feature they wouldn't recognise
