# Setting up your machine

Everything you need to install for the course, by operating system, and what to do when it goes wrong. Most of the course needs only Rust. Week 1's Tauri spike and week 6's database need a little more, and you can install those when you get there.

| Week | What it needs |
| --- | --- |
| 1 onwards | Rust (through `rustup`), Git, an editor with rust-analyzer |
| 1 (the Tauri spike), 5 (to run the app) | Node.js, Tauri's system libraries, and Android Studio for the phone build |
| 6 onwards (0x0B, and 0x0E's checks) | A Postgres server: Docker is the easiest way |
| 7 (0x0C) | `curl`, to call the server by hand |

The projects in this repository themselves are plain Rust: 0x0A's Tauri code is tested without the Tauri crate, and the database driver uses no OpenSSL or other C libraries. So `cargo test` only ever needs Rust and a linker, plus Postgres for 0x0B.

## 1. Install Rust with rustup

`rustup` installs Rust and keeps it up to date, and it's what reads the version this course pins. Don't install Rust from your system's package manager (`apt install rustc`, `brew install rust`): those don't understand `rust-toolchain.toml`.

### Linux

You need a C compiler first, because Rust uses the system linker:

```bash
# Debian, Ubuntu, Mint
sudo apt update && sudo apt install build-essential curl
# Fedora
sudo dnf install gcc curl
# Arch
sudo pacman -S base-devel curl
```

Then install rustup and accept the default options:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Close and reopen your terminal (or run `. "$HOME/.cargo/env"`) so `cargo` is on your `PATH`.

### macOS

Install Apple's command-line tools (they include the linker), then rustup:

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

1. Install the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and tick **Desktop development with C++**. Rust's default Windows toolchain (`x86_64-pc-windows-msvc`) links with Microsoft's linker, `link.exe`, which comes from there.
2. Install rustup: download `rustup-init.exe` from [rustup.rs](https://rustup.rs/) and run it, or run `winget install Rustlang.Rustup`. Accept the defaults.
3. Open a **new** terminal (PowerShell or Windows Terminal) so it picks up the new `PATH`.

If you'd rather not install the Build Tools and chose the GNU toolchain instead, install [WinLibs](https://winlibs.com/) (`winget install BrechtSanders.WinLibs.POSIX.UCRT`) and add its `mingw64\bin` folder to your `PATH`, or the build fails from week 3 with errors about `dlltool`.

### WSL (Windows Subsystem for Linux)

WSL works well for this course, and Docker Desktop's Postgres is reachable from it on `localhost`. Inside your WSL terminal, follow the **Linux** steps above: WSL is a separate Linux system and doesn't use a Rust installed on Windows.

Two things to know:

- **Keep the course inside the Linux file system** (`~/learning-rust`), not under `/mnt/c/...`. Builds on `/mnt/c` are many times slower.
- **Use VS Code's WSL extension**: run `code .` from the course folder in WSL, and rust-analyzer runs inside Linux, where your Rust is.

The Tauri spike is easiest on Windows itself, not in WSL: a Tauri app built in WSL needs WSLg to show a window, and can't reach an Android emulator running on Windows.

## 2. The course's pinned Rust version

`rust-toolchain.toml`, at the top of the course, pins one exact Rust version with `rustfmt` and `clippy`, so everyone (and CI) gets the same compiler and the same lints:

```toml
[toolchain]
channel = "1.99.1"
components = ["rustfmt", "clippy"]
```

Inside the course folder, `rustup` switches to that version by itself. Install it once, up front:

```bash
cd learning-rust
rustup toolchain install   # installs the version rust-toolchain.toml asks for
rustup show                # "active toolchain" should say 1.99.1, overridden by rust-toolchain.toml
cargo --version
cargo clippy --version
```

Any `cargo` command in the folder also downloads it the first time, so the first `cargo test` can pause for a minute. Outside the course folder you keep whatever Rust you had (usually `stable`).

When the course moves to a newer version, `rust-toolchain.toml` changes, and the next `cargo` command fetches it. To clear out old ones: `rustup toolchain list`, then `rustup toolchain uninstall <version>`.

## 3. Your editor: rust-analyzer

rust-analyzer shows errors as you type, the type of every variable, and jumps to definitions. It makes learning Rust much faster.

- **VS Code** (or VSCodium, Cursor): install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer). Don't also install the old "Rust" extension (`rust-lang.rust`): it's deprecated and the two fight.
- **RustRover** (JetBrains): Rust support is built in.
- **Zed**: Rust support is built in.
- **Neovim, Helix, Emacs**: install the server with `rustup component add rust-analyzer`, then point your editor's language-server setup at it.

Open the **course folder** itself (the one with the top-level `Cargo.toml`), not a single project's folder: it's one Cargo workspace, and rust-analyzer needs the top to see it.

Worth turning on in VS Code's settings: **Format on save** (runs `rustfmt`), and `rust-analyzer.check.command` set to `clippy`, so you see clippy's warnings as you type instead of only when you run it.

## 4. Git

Check with `git --version`. If it's missing: `sudo apt install git` (Linux), `xcode-select --install` (macOS, already done above), or [git-scm.com/downloads](https://git-scm.com/downloads) (Windows). Then:

```bash
git clone https://github.com/sagebeme/learning-rust.git
cd learning-rust
cargo test -p hello-cargo   # every test fails with "not yet implemented": that's expected
```

On Windows, Git may offer to convert line endings. The course has a `.gitattributes` that keeps every file with Unix line endings (LF), so either answer is fine, and rustfmt won't report every line as changed.

## 5. For the Tauri spike (week 1) and running the app (week 5)

You only need these to build and run the Tauri app itself. The tests in 0x0A don't.

1. **Node.js 20 or newer** (with npm): the LTS version from [nodejs.org](https://nodejs.org/). Check with `node --version`.
2. **Tauri's system libraries**: follow Tauri's [prerequisites guide](https://v2.tauri.app/start/prerequisites/) for your system. In short:
   - **Debian or Ubuntu:**
     ```bash
     sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
       libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
     ```
     Other distributions are in the guide. Tauri 2 needs WebKitGTK **4.1**: very old releases (Ubuntu 20.04) only have 4.0.
   - **macOS:** the command-line tools from step 1 are enough for the desktop app. Building for iOS needs the full Xcode from the App Store.
   - **Windows:** the Build Tools from step 1, plus the WebView2 runtime (Windows 10 version 1803 and later, and Windows 11, already have it).
3. **For the Android build:** [Android Studio](https://developer.android.com/studio), and in its SDK Manager: an Android SDK Platform, Platform-Tools, Build-Tools, Command-line Tools and the **NDK (Side by side)**. Set `JAVA_HOME` (to Android Studio's bundled JDK), `ANDROID_HOME` and `NDK_HOME` as the Android section of the prerequisites guide shows, then add Rust's Android targets:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```
   Run that from the folder of the Tauri app you're building: targets are added to the Rust version active there. Leave about 15 GB free and start these downloads early.

The [week 1 guide](week-1-tauri.md) walks you through creating and running the app.

## 6. For the database (week 6 onwards)

0x0B's tests need a Postgres server and a `DATABASE_URL`. The easiest way is [Docker](https://docs.docker.com/get-started/get-docker/) (Docker Desktop on Windows and macOS, Docker Engine on Linux). Check with `docker run hello-world`, then start Postgres 17, the version CI uses:

```bash
docker run --name twa-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:17
```

Next time, `docker start twa-postgres` brings it back with its data. Then, in the terminal you'll run the tests from:

```bash
# bash / zsh (Linux, macOS, WSL)
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
# PowerShell
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres"
```

Prefer no Docker? [Install PostgreSQL](https://www.postgresql.org/download/) directly, set a password for the `postgres` user, and use the same `DATABASE_URL`. The `postgres` user needs permission to create databases, because every test makes its own.

That password is only for a database on your own machine with fake data. Never put a real database's password in the course, or in a commit. If you keep settings like this in a `.env` file, the course's `.gitignore` already keeps it out of Git.

## 7. `curl` (week 7)

0x0C's `src/main.rs` shows `curl` commands for calling the server. `curl --version` should already work on Windows 10 and later, macOS and most Linux; otherwise `sudo apt install curl`, or [curl.se/download](https://curl.se/download.html). In PowerShell, type `curl.exe`, not `curl`: plain `curl` there is a different command (`Invoke-WebRequest`) with different options.

## Troubleshooting

**`cargo: command not found`, or `'cargo' is not recognized`**
The terminal was opened before Rust was installed. Open a new one. On Linux and macOS, check `~/.cargo/bin` is on your `PATH` (`. "$HOME/.cargo/env"` adds it for now; the installer adds it to your shell's startup file).

**`error: linker 'cc' not found`** (Linux)
No C compiler. Install `build-essential` (Debian/Ubuntu), `gcc` (Fedora) or `base-devel` (Arch), as in step 1.

**`linker 'link.exe' not found`, or `error calling dlltool`** (Windows)
The C toolchain is missing. Install the Visual Studio Build Tools with **Desktop development with C++**, or WinLibs if you use the GNU toolchain (step 1). Open a new terminal afterwards.

**`xcrun: error: invalid active developer path`** (macOS)
Apple's command-line tools are missing, often after a macOS update. Run `xcode-select --install`.

**The first `cargo` command hangs on "syncing channel updates" or "downloading component"**
It's downloading the pinned Rust version, about 300 MB. Let it finish. Behind a company proxy, set `HTTPS_PROXY` first.

**`error: toolchain '1.99.1-...' is not installed`**
Run `rustup toolchain install` from the course folder. If it says your rustup is too old, run `rustup self update` first.

**`cargo clippy` or `cargo fmt`: "is not installed for the toolchain"**
Run `rustup component add clippy rustfmt` from the course folder, so they're added to the pinned version.

**rust-analyzer shows nothing, or "failed to discover workspace"**
You opened a project folder (or `src/`) instead of the course folder. Open the folder with the top-level `Cargo.toml`. If it still fails, run "rust-analyzer: Restart server" from the command palette, and check its output panel.

**`Blocking waiting for file lock on build directory`**
Another `cargo`, usually rust-analyzer, is building at the same moment. Wait a few seconds. If it never ends, close other terminals running `cargo`.

**The disk fills up**
`target/` holds every build and can grow to several GB. `cargo clean` deletes it safely; the next build is just slower.

**0x0B: "error communicating with database", "connection refused" or "password authentication failed"**
Postgres isn't running (`docker ps` should list `twa-postgres`; `docker start twa-postgres` if not), `DATABASE_URL` isn't set in *this* terminal, or the password in it doesn't match the one the server was started with. Port 5432 already taken by another Postgres? Start the container with `-p 5433:5432` and use port `5433` in `DATABASE_URL`.

**Docker: "permission denied while trying to connect to the Docker daemon socket"** (Linux)
Add yourself to the `docker` group (`sudo usermod -aG docker $USER`), then log out and back in.

**Tauri on Linux: `webkit2gtk-4.1` not found, or `pkg-config` errors**
The system libraries from step 5 are missing. Install them, then build again.

**Tauri Android: "NDK not found" or "ANDROID_HOME is not set"**
Set `ANDROID_HOME` and `NDK_HOME` as in step 5, and open a new terminal. On Windows, set them in **System Properties → Environment Variables**, not just in one terminal.

**Builds are very slow on WSL**
The course is under `/mnt/c`. Move it into your Linux home folder (`~/`), as in step 1.

Still stuck? Copy the **whole** error message and search for it, or ask your mentor. For compiler errors in your own code, see [common compiler errors](common-errors.md).
