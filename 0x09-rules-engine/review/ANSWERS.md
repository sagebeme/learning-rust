# Answers

Ordered by how much damage each one does.

1. **Grace, warnings, recency, prerequisites and equivalents are all ignored.** Someone in their grace period is "expired" and gets pulled from shifts; someone whose prerequisite has expired is "ok"; the external DG certificate is never accepted. The PR implements a different, simpler rule than the business asked for. Ask which requirements it's meant to cover, and link to the agreed rules.

2. **Months as 30 days.** 24 months becomes 720 days, so a two-year licence "expires" about ten days early. Use calendar months (`checked_add_months`), as the business defines validity.

3. **It reads the local clock.** `chrono::Local::now()` depends on the device's time zone and clock. A phone set to the wrong zone, or a server running in UTC, gives a different answer from the same data. Take `today` as a parameter.

4. **It returns a `HashMap<String, String>`.** The order is random, so two runs of the same check print differently. The states are free text, so a typo like `"exipred"` compiles. And there's no room for detail such as days left or what's missing. Return the typed `GapReport` with `RequirementState`, in code order.

5. **`catalogue[code]` panics** if the matrix mentions a code the catalogue doesn't have, which takes down the whole gap check, or the server request. Return `EngineError::UnknownQualification` instead.

6. **One holding per code** (`HashMap<String, NaiveDate>`). Someone with an old *and* a renewed certificate can only be represented by one of them, and whoever builds the map decides which, silently. The engine should receive every holding and choose the best one itself.

7. **`as i64`** on `validity_months` is harmless here (`u32` always fits in `i64`), but `i64::from` says so and never needs checking. It's a nit: say it's a nit.

8. **"Tested with Amina's transfer"**: one real person's case is a good *fixture*, but it's real staff data. Ask for it to be rewritten with fake data and added to the shared `fixtures/transfers.json`, along with boundary cases.

Suggested reply opening: *"Thanks for getting this started. Before it goes in, it needs to follow the agreed rules for grace, recency and prerequisites (1), use calendar months (2) and take today as a parameter (3), or devices will disagree with each other. Details below."* Kind, specific, and the blockers first.
