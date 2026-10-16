# TWA architecture

One page. If it doesn't fit, it's too detailed for this document; link to a decision record instead.

## What TWA does

<!-- Two sentences: who uses it, and the main jobs it does for them. -->

## The pieces

| Piece | Built with | Responsible for | Must never |
| --- | --- | --- | --- |
| Rules engine (`twa-engine`) | Rust library | Qualification rules and gap checks | Read the clock, the network or the database |
| App (Tauri) | Rust core + web screens | Showing status, working offline, collecting input | Decide who is qualified; hold secrets |
| Server (`twa-server`) | Rust, Axum | The authority for records; transfers; sync | Trust input from the app without checking it |
| Database | Postgres (Supabase) | Durable records; enforcing keys and checks | |
| Daily job | Rust | Finding expiries; sending alerts once | Send twice |
| <!-- add more --> | | | |

## How data flows

<!-- Follow one real path end to end, in numbered steps. For example: a training officer records a
     renewed DG certificate → ... → Amina's app shows "valid until March 2028", and no reminder is
     sent tomorrow. -->

## Where each rule is enforced

| Rule | Enforced in |
| --- | --- |
| Staff numbers are unique | Database primary key |
| Older records never replace newer ones | <!-- --> |
| Validity, grace, recency | <!-- --> |
| Only the training office can record certificates | <!-- --> |

## When things go wrong

- **A station is offline for a day:** <!-- -->
- **The SMS provider is down:** <!-- -->
- **The server crashes mid-transfer:** <!-- -->

## Decisions

<!-- One line per decision record, linked. -->
