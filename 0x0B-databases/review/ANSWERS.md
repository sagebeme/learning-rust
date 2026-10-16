# Answers

1. **SQL injection.** `format!` puts `station` straight into the SQL text. A value like `NBO' OR '1'='1` returns every employee in the company, and worse is possible. Always use placeholders: `WHERE station_code = $1` with `.bind(station)`. This is a blocker, whatever the PR description says about where `station` comes from today.

2. **N+1 queries.** One query for the employees, then one more *per employee*. At a station with 300 staff that's 301 round trips to the database, and on a slow link from a station it adds up fast. One `JOIN` does it in a single query:

   ```sql
   SELECT e.staff_number, e.name, h.code
   FROM employees e JOIN holdings h ON h.staff_number = e.staff_number
   WHERE e.station_code = $1
   ORDER BY e.staff_number, h.code
   ```

3. **No `ORDER BY`.** Postgres returns rows in whatever order is convenient, and it can change between runs. Anything shown to people or compared in tests needs an explicit order.

4. **`renew` deletes, then inserts, with no transaction.** If the insert fails (a bad code, a dropped connection, a crash in between), the person's qualification is simply *gone*. Use a transaction, or better, a single upsert (`INSERT ... ON CONFLICT ... DO UPDATE`), which can't half-happen.

5. **`.ok()` throws every error away.** `renew` can fail completely and the caller is told nothing. Return `Result<(), DbError>`.

6. **`unwrap()` in `holdings_for_station`** turns a database hiccup into a crashed server request. Return a `Result`.

7. **The validity rule is in SQL** (`+ interval '24 months'`). Validity belongs to the qualification type, and the engine already calculates it, with the month-end rules agreed with the business. Calculate `valid_until` with the engine and store the result, so the database and every device agree.

8. **Tuples like `(i32, String, String)`** make callers guess which `String` is which. Return a struct deriving `FromRow`.

Order for the review: 1 (security) and 4 (data loss) are blockers; 2 and 7 matter before launch; the rest are clean-up.
