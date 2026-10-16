# Week 1: the client app is Tauri

**Owner:** you, as the architect · **Due:** end of week 1 · **Output:** a working spike, and `docs/decisions/0001-client-app.md` filled in

The team has chosen **Tauri 2** for the TWA client app. This week you make that choice real: set up the toolchain, prove a Rust function can be called from the app on desktop and on a phone, and write the decision record you'll point to when anyone asks "why Tauri?".

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| Rust, set up for this course | `cargo test -p hello-cargo` runs in this repo | Follow "Before you start" in the [main README](../README.md#before-you-start) |
| Node.js 20 or newer, with npm | `node --version` and `npm --version` print versions | Install the LTS version from [nodejs.org](https://nodejs.org/) |
| Tauri's system prerequisites | Step 2 of the spike builds and opens a window | Tauri's [prerequisites guide](https://v2.tauri.app/start/prerequisites/) for your operating system. On Windows that's the WebView2 runtime (Windows 11 already has it) and a C toolchain |
| Android Studio, with the SDK and NDK | Android Studio opens, and its SDK Manager lists an NDK as installed | [Android Studio](https://developer.android.com/studio), then the Android section of the prerequisites guide (it sets `ANDROID_HOME` and `NDK_HOME`) |
| A Mac with Xcode, for iOS | Only if you have one | Not blocking: iOS is optional this week. Record who on the team owns a Mac |
| A few hours of disk and download time | About 15 GB free | Android Studio, the SDK and emulator images are large: start the downloads on day 1 |

## The story so far

TWA will answer one question all through this course: *is this person qualified to do this job, at this station, today?* Your first answer is tiny: a Rust function that labels how many days a licence has left. This week you prove that answer can reach a screen, on a laptop and on a phone.

## What Tauri is, in one paragraph

A Tauri app has two halves. The **core** is a Rust program: it owns the data, the rules engine, files and the database. The **screens** are web code (HTML, CSS and TypeScript, for example React) shown in the operating system's own web view. The screens ask the core to do things by calling **commands**: Rust functions marked `#[tauri::command]`. One codebase builds for Windows, macOS, Linux, Android and iOS.

## Why it fits TWA

- **The rules engine is plain Rust inside the core.** No bridge layer to generate or maintain: the screens call a command, and the command calls the engine.
- **Every platform from one codebase, Linux included.**
- **The screens use the same web skills as the admin app.** Components, and even TypeScript types for the data, can be shared.
- **Small apps.** Tauri uses the web view already on the device instead of bundling a browser.

## What it costs, and what to watch

Write these into the "Consequences" section of your decision record, in your own words:

- **Four different web views.** WebView2 on Windows, WebKit on macOS and iOS, WebKitGTK on Linux, Android System WebView on Android. A screen can look or behave slightly differently on each, so test on all of them, not just your laptop.
- **Mobile support is younger than desktop.** Check which plugins you need (camera, notifications, file access, secure storage) support Android *and* iOS before promising a feature.
- **iOS builds need a Mac** with Xcode. Plan who owns one.
- **Security is opt-in, per command.** Tauri 2 only lets the screens call what you've allowed in its capabilities (permissions) files. That's good, but it means every new command is a security decision, and a code-review item.
- **Your Rust is now on the user's device.** Anything in the core can be inspected, so secrets (API keys, the database password) never go in the app. They stay on the server.

## The spike

One to two days, throwaway code. Keep it **outside this repo's workspace**, for example in `../twa-tauri-spike`, so it never affects `cargo test` here.

1. **Install the prerequisites** from the table above.
2. **Create an app** with `npm create tauri-app@latest` and pick React with TypeScript, then `npm install` and `npm run tauri dev` in the new folder. A window should open. If it does, every desktop prerequisite is working.
3. **Add one command** that calls a function you wrote this week, such as `expiry_label` from `0x01`:

   ```rust
   #[tauri::command]
   fn expiry_label(days_left: i64) -> String {
       // your 0x01 logic here
   }
   ```

   Register it with the builder's `invoke_handler`, and call it from the screen with `invoke("expiry_label", { daysLeft: 12 })`. Notice the screen's `daysLeft` became Rust's `days_left`: Tauri converts the names for you.
4. **Show a list of five fake staff** with their days left, each labelled by your Rust command.
5. **Run it on your desktop, then on an Android emulator or phone.** Run `npm run tauri android init` once, to generate the Android project, then `npm run tauri android dev`. Add iOS if you have a Mac (`npm run tauri ios init`, then `npm run tauri ios dev`).
6. **Write down:** time to the first working build on each platform, anything that needed a workaround, and the size of the built app.

## Then write the decision record

Fill in `docs/decisions/0001-client-app.md`. The decision is already made, so your job is the reasoning and the costs, written clearly enough that a new team member, or a client, understands it without you in the room.

## Questions to take to your next client meeting

The spike will raise these. Ask them early, because the answers change the build:

- Which devices do staff actually use at each station? Company phones, shared tablets, desktop PCs in the training office?
- Are company devices managed (MDM)? Can we install apps directly, or only through the Play Store and App Store?
- How long can a station go without internet? Hours, or days?
- Do any staff use Linux desktops today, or is Linux for the future?

## Read more

- Tauri 2: [Create a project](https://v2.tauri.app/start/create-project/), [Calling Rust from the frontend](https://v2.tauri.app/develop/calling-rust/), and [Develop for mobile](https://v2.tauri.app/develop/#developing-your-mobile-application)
- Michael Nygard's [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions), the short essay that decision records come from

## Checkpoint

- The spike runs on your desktop and on Android, calling your Rust command
- The decision record is filled in, including the costs above
- You can explain "why Tauri" to a non-technical manager in under two minutes
