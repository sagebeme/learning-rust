# 0x0E. Architecture, reviews and clients

**Week 8** · The parts of your job that aren't writing Rust: deciding how TWA fits together, reviewing other people's code, and explaining all of it to the client.

You now know enough Rust to build every part of TWA. This last week is about the decisions around the code, the ones that are expensive to change in month three.

## Before you start

| You need | Check | If it's missing |
| --- | --- | --- |
| Weeks 5–7 done | `cargo test -p tauri-core -p async-backend -p jobs-alerts` passes, and `cargo test -p databases` against your Postgres | Go back to the first project that fails |
| Your week 1 decision record | `docs/decisions/0001-client-app.md` exists in your notes | Write it now, from [the week 1 guide](../docs/week-1-tauri.md) and [the template](../docs/decisions/) |
| A mentor to play the client | Someone has agreed to 30 minutes, with the worksheet in hand | Ask your lead; a colleague reading [client-meetings.md](../docs/client-meetings.md) first works too |
| A GitHub account, for reading real PRs | You can open [github.com/storytold](https://github.com/storytold) | [Sign up](https://github.com/signup); the course itself only needs Git |

## The story so far

Amina's question now gets answered everywhere: in a test (0x09), in the app (0x0A), from the database (0x0B), by the server (0x0C), and at 06:00 by WhatsApp (0x0D). Every piece uses the same engine. This week you make sure it stays that way, check other people's changes to it, and show it to the client.

## What you'll be able to do

- Describe TWA's architecture on one page, and turn its key rules into tests that run in CI
- Write decision records that someone joining in a year can follow
- Review a pull request quickly and usefully: blockers first, every comment with a reason
- Run a client meeting that turns vague requirements into exact, testable rules

## Read first

- Your own week 1 decision record (`docs/decisions/0001-client-app.md`). Does it still hold?
- [docs/code-review-guide.md](../docs/code-review-guide.md) and [docs/client-meetings.md](../docs/client-meetings.md)
- storytold's [`craft-libs` README](https://github.com/storytold/craft-libs), and the "Backend architecture" section of [`artcraft-services`](https://github.com/storytold/artcraft-services): real Rust workspaces, organised the way this week describes

**Go deeper:** Google's [engineering practices guide to code review](https://google.github.io/eng-practices/review/), Michael Nygard's [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions), and Kenya's [Office of the Data Protection Commissioner](https://www.odpc.go.ke/) for decision 0002.

## Part 1: architecture you can test (code)

```bash
cargo test -p architecture-clients
```

### 0. Layer rules (`t00_layer_rules.rs`)

Architecture rules on a diagram get broken quietly ("just this once, the engine reads the database"). Rules in a test fail the build. Write `check_layers`: compare what each crate depends on with what it's allowed to depend on. For TWA:

| Crate | May depend on | Why |
| --- | --- | --- |
| `twa-engine` | nothing internal | Pure rules: runs identically in the app and on the server, and tests need no setup |
| `twa-db` | `twa-engine` | Stores what the engine works with |
| `twa-server` | `twa-engine`, `twa-db` | The only place everything meets |
| `twa-app-core` (Tauri) | `twa-engine` | The app works offline from synced data; it never talks to Postgres directly |

### 1. Read the manifests (`t01_read_manifest.rs`)

A small reader for `Cargo.toml` that finds the package name and its internal dependencies. Real tools use `cargo metadata`; writing a small one yourself shows what the real one gives you.

### The test that uses both

`tests/this_repo.rs` runs your two functions over **this** workspace and checks the course's own rules: the weeks 1–4 projects depend on nothing internal, and the weeks 5–7 projects may depend on the engine (the reference one or yours), and on nothing else. That's the TWA table above in miniature. When TWA's workspace exists, copy the test there with TWA's rules.

## Part 2: write the architecture down

Copy [docs/templates/architecture.md](../docs/templates/architecture.md) to your notes and fill it in for TWA: the pieces, how data flows from a training record to an SMS, where each rule is enforced, and what happens when a station is offline.

Then write three more decision records, using [docs/decisions/](../docs/decisions/) as a starting point:

- **0002: database and hosting.** Supabase-hosted Postgres? Where do backups go? What happens to staff data, under Kenya's Data Protection Act?
- **0003: offline sync.** Which data each device keeps, how changes get back to the server, and what wins when two stations disagree. (Hint: the server, using the "never go backwards" rule from 0x0B task 1.)
- **0004: alert providers.** Which SMS, WhatsApp, email and push services, what they cost per message, and what happens when one is down.

## Part 3: review five pull requests

Open [`review/README.md`](review/README.md): five realistic PRs, one per area of TWA. Give yourself 15 minutes each, as on a real working day. Write the comments you'd post, then compare with `review/ANSWERS.md`.

## Part 4: the client meeting

Use [docs/client-meetings.md](../docs/client-meetings.md) to prepare and run a 30-minute demo of your mini-TWA to your mentor, playing the training manager:

1. Show Amina's transfer check, live. Start the server with `cargo run -p async-backend`, ask it the question with the `curl` command in `0x0C-async-backend/src/main.rs`, and explain each line of the answer in plain words.
2. Walk through what happens at 06:00 every day: run `cargo run -p jobs-alerts --example morning_run` and show who gets told, how, and why running it twice sends nothing new.
3. Ask the five open questions from the worksheet, and write the answers as rules plus fixtures cases.

## Check yourself

Answer each one before you open the answers.

1. **What prints?** A first try at reading a dependency line in `t01_read_manifest.rs`:

   ```rust
   fn main() {
       let line = r#"twa-engine = { path = "../engine-reference" }"#;
       println!("{:?}", line.split_once('='));
       let (name, _) = line.split_once('=').unwrap_or_default();
       println!("{}", name == "twa-engine");
   }
   ```

2. **Spot the problem.** A pull request adds `twa-db` to `twa-app-core`'s dependencies "so the app can show the latest records". Which of TWA's layer rules does it break, and what's the cost of letting it through?
3. **Why** are the layer rules a test, when they're already drawn on the architecture diagram?
4. **Sort these review comments** into blockers and suggestions:
   - a. "This query puts the station name into the SQL with `format!`."
   - b. "I'd call this `days_left` rather than `remaining`."
   - c. "This handler holds the lock while it calls the engine."
   - d. "Could this loop be an iterator chain?"
5. **Turn this into a rule you could test:** the client says *"people should renew their certificates in good time"*. What do you ask, and what does the rule look like afterwards?
6. **Why** does a decision record say what you *didn't* choose, and why?

<details>
<summary>Answers</summary>

1. `Some(("twa-engine ", " { path = \"../engine-reference\" }"))`, then `false`. Both halves keep their spaces, so the name is `"twa-engine "` and the comparison fails. Trim both halves. A test with a real `Cargo.toml` line catches this straight away.
2. `twa-app-core` may only depend on `twa-engine`: the app works offline from synced data and never talks to Postgres directly. Letting it through puts database credentials on staff phones, breaks the offline mode, and gives the app a second, different way to get records. Your `check_layers` should fail the build on it.
3. A diagram doesn't stop anyone. "Just this once" changes are made under time pressure, and nobody checks the drawing. A test fails the build the moment the rule is broken, and it explains the rule to whoever broke it.
4. Blockers: a (SQL injection) and c (it freezes every other request that needs the lock). Suggestions: b and d, which are about style. Say which is which, and give the reason with each comment.
5. Ask: how long before expiry counts as "in good time" (30 days? 60?), does it differ by qualification, who is told, and what happens when they don't. Afterwards: "a DG certificate enters the warning window 60 days before it expires; the person is reminded at 60, 30, 7 and 1 days", with a fixtures case for someone on day 61 and day 60.
6. So that someone joining in a year knows the other options were considered, and why they lost. Without it, they propose the rejected option again, or undo the decision without knowing what it was protecting.

</details>

## On your own

No steps, no tests given, no solution. Put your test in `tests/on_your_own.rs`, built on your task 0 and 1 functions and on how `tests/this_repo.rs` reads the workspace.

**The brief.** The layer rules cover the course's own crates. Some rules are about outside crates too, and they matter as much: only the database layer talks to Postgres, and only the server runs a web framework. Write a test that checks, for every project in this repo:

- only `0x0B-databases` depends on `sqlx`
- only `0x0C-async-backend` depends on `axum`
- nothing from weeks 1–5 (0x00 to 0x0A) or `engine-reference` depends on `tokio`

Then write decision record 0005, explaining the rule in the format of `docs/decisions/`.

**Constraints**

- Extend your manifest reader instead of searching the file for a word: `sqlx` in a comment, or a crate called `sqlx-helpers`, isn't a dependency.
- `[dependencies]` and `[dev-dependencies]` both count.
- When the test fails, its message names the project, the crate, and the rule it broke.

**Done when**

- The test passes on this repo
- Adding `tokio.workspace = true` to `0x0A-tauri-core/Cargo.toml` makes it fail with a message that says what to do (then take the line out again)
- Unit tests for your reader cover a comment, a similar-looking crate name, and a dev-dependency
- The decision record says what you'd do when a week-5 project genuinely needs async

## Checkpoint: the end of the course

- `cargo test -p architecture-clients` passes, and clippy is clean
- TWA's architecture page and decision records 0001–0004 are written
- Your five reviews are done, and you've compared them with the answer key
- You've given the demo, and your mentor would trust you in front of the client

## When you're stuck

- **Not sure where a piece of code belongs**: ask "does it need the network, the database or the clock?" If not, it belongs in the engine.
- **A decision record feels too long**: it should fit on one screen. Context, decision, consequences: the reasoning, not the research.
- **A review comment feels harsh**: rewrite it as *what*, *why* and *what instead*. Facts are never harsh; tone can be.
