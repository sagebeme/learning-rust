# 0x09. The rules engine

**Week 4** · The capstone of the foundations: a small, complete, tested version of the engine at the heart of TWA.

When someone transfers to a new department, TWA asks one question: *what does this person still need?* This project answers it. Everything from the last four weeks goes in: enums that make bad states impossible, `Result` errors, collections in a fixed order, dates, traits, and tests, including property tests.

When you finish, you've done the foundations. The rest of the course puts this engine inside an app, a database and a server.

## What you'll be able to do

- Turn written business rules (validity, grace, warning, recency, prerequisites, equivalents) into code that's exact about every boundary
- Build a bigger feature out of small, separately tested functions
- Prove the engine gives the same answer whatever order its input arrives in

## Read first

- Re-read your own 0x04 (`QualificationStatus`), 0x06 (`BTreeMap`) and 0x08 (dates, property tests)
- The `serde` docs: "Derive", and "Enum representations" (the `tag` attribute)

## The model (`src/model.rs`, given)

Read it before writing anything. The key types:

| Type | What it is |
| --- | --- |
| `QualificationType` | A kind of qualification: how long it lasts, grace and warning periods, recency, prerequisites, equivalents |
| `Holding` | One person's record of one qualification: when they completed it, and when they last used it |
| `RequirementState` | Where a requirement stands: `Met`, `ExpiringSoon`, `InGrace`, `Lapsed`, `Expired`, `PrerequisiteMissing` or `Missing` |
| `Catalogue`, `RequirementsMatrix` | Every qualification type, and what each department and role needs |
| `GapReport` | The answer: one line per requirement |

Everything derives `Serialize` and `Deserialize`, so the same data can be sent as JSON to the app's screens (0x0A), stored (0x0B) and returned by the server (0x0C).

## Tasks

Do them in order: each one uses the one before.

```bash
cargo test -p rules-engine
```

### 0. Valid until (`t00_valid_until.rs`)

Your `add_months` from 0x08, applied to a `QualificationType`.

### 1. One holding's state (`t01_holding_state.rs`)

The heart of the engine. Follow the order in the file's comment exactly: **expiry first, then recency, then the warning window**. The order is a business decision. A licence in its grace period that hasn't been used recently is reported as "in grace", because the expiry is the more urgent fact. The tests check every boundary: the last valid day, the last grace day, day 90 vs day 91 of recency.

### 2. The best holding (`t02_best_holding.rs`)

Someone may hold the qualification itself *and* an equivalent. Work out each one's state and keep the best. Build a sort key as a tuple, `(rank, Reverse(valid_until), code)`, and keep the smallest. `Reverse` flips the order, so the *latest* date sorts first. Tuples compare left to right, which gives you the three tie-break rules in one comparison, and makes the answer independent of input order.

### 3. Prerequisites (`t03_requirement_state.rs`)

Recursive: a prerequisite is itself a requirement, with equivalents and prerequisites of its own. Note the subtle rule tested in the last test: an *equivalent* still has to meet the *required* type's prerequisites. Holding the external DG certificate doesn't excuse anyone from airside safety.

### 4. The gap check (`t04_gap_check.rs`)

Look up the role, deduplicate and sort the codes with a `BTreeSet`, and build one line each. The last test is a property test: random holdings, checked forwards and reversed, must give identical reports.

### 100. Validate the catalogue (`t100_validate_catalogue.rs`) — advanced

Your 0x07 cycle detection, aimed at real data. Without it, a catalogue where A requires B and B requires A would send task 3 into infinite recursion. Validating at the edge, once, is what lets the core trust its input.

## Shared test cases: same answer everywhere

`tests/fixtures/transfers.json` holds whole transfer scenarios, written as data: the holdings, the role, and the exact expected report. `tests/conformance.rs` runs every one of them.

On TWA, the same engine runs in the app on five platforms and on the server. A shared file of cases like this is how you prove they all agree: every build runs the same file and must produce the same output. When a client reports "the app says one thing and the dashboard says another", the first step is to add their case to this file.

Add a case of your own: a ramp agent whose fire safety certificate expires tomorrow.

## Review it

Open [`review/README.md`](review/README.md): a gap check that works on the author's example and is wrong in four ways.

## Explain it

Write the explanation of a gap report you'd give to a training manager, using case 2 from the fixtures file. Why can't this person start in Cargo, even though their DG certificate is "in grace"? What do they need to do, in what order? Three or four sentences, no code words.

## Checkpoint: end of the foundations

- `cargo test -p rules-engine` passes, including the fixtures and the property test, and clippy is clean
- You've added your own fixtures case
- **Ask for a code review** of this project from your mentor or a teammate, as if it were a real pull request. Make the changes they ask for.

## When you're stuck

- **A boundary test fails by one day**: check whether you used `>` or `>=`. Each one is a business decision: re-read the rule in the file's comment.
- **Infinite recursion in task 3**: the catalogue in your test has a cycle. Task 100 exists to stop that.
- **The property test fails**: read the minimal failing input it prints. The usual cause is choosing between equal candidates by input order instead of by the sort key.
- **"the trait `Serialize` is not implemented"**: add it to the derive, and check the `serde` feature is on for `chrono` (it is, in the workspace `Cargo.toml`).
- **Still stuck?** Read `solution/`, close it, and write it again from memory.
