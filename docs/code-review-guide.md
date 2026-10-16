# How to review a pull request

On TWA you'll review more code than you write. A good review protects staff (the engine decides who may work airside), the client's data, and your teammates' time. This is the method every `review/` exercise in the course practises.

## Before you read the code

1. **Read the description.** What problem does it solve? Is there a ticket or a client request behind it? If you can't tell what it's *for*, ask before reviewing anything else.
2. **Check that CI is green.** Formatting, clippy and tests are the machine's job, not yours.
3. **Look at the size.** Over about 400 changed lines, ask for it to be split. Big PRs get skimmed, and skimmed PRs let bugs through.

## Read in this order

| Order | Look for | Typical blocker |
| --- | --- | --- |
| 1 | **Correctness of the rules** | A business rule invented or changed in code without a source; boundaries (`<` vs `<=`) untested |
| 2 | **Security and personal data** | SQL built with `format!`, secrets in the app, staff data sent or logged where it shouldn't be |
| 3 | **Data safety** | Missing transactions, errors swallowed (`.ok()`, `let _ =`), older data overwriting newer |
| 4 | **Failure behaviour** | `unwrap` where input can be bad, panics in commands or handlers, retries without limits |
| 5 | **Determinism** | `HashMap` order in output, reading the clock inside rules, results depending on input order |
| 6 | **Tests** | Do they cover the boundaries and the failure cases, or only the happy path? |
| 7 | **Design and naming** | Is the code in the right crate (see the layer rules in 0x0E)? Will the next reader understand it? |
| 8 | **Style** | Only what the formatter and clippy don't catch, and mark it as a nit |

## Writing comments

Every comment has three parts: **what**, **why**, **what instead**.

> **Blocker:** `fs::write(...)` here ignores its result (`let _ =`), so a full disk silently loses the station's settings and the function still reports success. Use `?` and let the caller show the error.

Label every comment **blocker**, **should fix** or **nit**, so the author knows what has to change before merging. Put blockers first.

Ask questions when you're not sure, rather than asserting: "What happens here if the station has no contact for this person?" Often the author knows something you don't.

Say what's good too, briefly. "The fixtures cases for the grace boundary are exactly right" tells the author what to keep doing.

## Approving

Approve when there are no blockers left, even if nits remain. Re-review quickly when changes come back: a PR waiting days for a second look costs more than most bugs.

## Reviewing with AI

AI tools can be useful reviewers, for summarising a large diff or spotting a missed error path. But you're accountable for the review. Never paste a comment you couldn't defend yourself, and never send real staff data or secrets to an outside tool.
