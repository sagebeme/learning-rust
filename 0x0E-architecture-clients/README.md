# 0x0E. Architecture, reviews and clients

**Week 8** · The parts of your job that aren't writing Rust: deciding how TWA fits together, reviewing other people's code, and explaining all of it to the client.

You now know enough Rust to build every part of TWA. This last week is about the decisions around the code, the ones that are expensive to change in month three.

## What you'll be able to do

- Describe TWA's architecture on one page, and turn its key rules into tests that run in CI
- Write decision records that someone joining in a year can follow
- Review a pull request quickly and usefully: blockers first, every comment with a reason
- Run a client meeting that turns vague requirements into exact, testable rules

## Read first

- Your own week 1 decision record (`docs/decisions/0001-client-app.md`). Does it still hold?
- [docs/code-review-guide.md](../docs/code-review-guide.md) and [docs/client-meetings.md](../docs/client-meetings.md)
- storytold's `craft-libs` README, and the "Backend architecture" section of `artcraft-services`: real Rust workspaces, organised the way this week describes

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

`tests/this_repo.rs` runs your two functions over **this** workspace and checks the rule for this course: no project depends on another. When TWA's workspace exists, copy the test there with TWA's rules.

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

1. Show a transfer gap check (0x09) for a fake staff member, and explain each line in plain words.
2. Walk through what happens at 06:00 every day (0x0D): who gets told, and how.
3. Ask the five open questions from the worksheet, and write the answers as rules plus fixtures cases.

## Checkpoint: the end of the course

- `cargo test -p architecture-clients` passes, and clippy is clean
- TWA's architecture page and decision records 0001–0004 are written
- Your five reviews are done, and you've compared them with the answer key
- You've given the demo, and your mentor would trust you in front of the client

## When you're stuck

- **Not sure where a piece of code belongs**: ask "does it need the network, the database or the clock?" If not, it belongs in the engine.
- **A decision record feels too long**: it should fit on one screen. Context, decision, consequences: the reasoning, not the research.
- **A review comment feels harsh**: rewrite it as *what*, *why* and *what instead*. Facts are never harsh; tone can be.
