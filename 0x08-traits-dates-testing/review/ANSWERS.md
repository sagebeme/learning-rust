# Answers

1. **It reads the clock itself (`Utc::now()`).** The answer changes depending on when you run it, so it can't be tested reliably, and two devices with slightly different clocks can disagree. Pass `today: NaiveDate` in as a parameter, and get "today" once, at the edge of the app.

2. **UTC, not Nairobi time.** Kenya is UTC+3, so between midnight and 3 a.m. in Nairobi, `Utc::now().date_naive()` is *yesterday*. For three hours every night, licences that expired at midnight still show as valid. Getting the date belongs at the edge too, in the station's own time zone.

3. **Years as `f64` times 365.** Two years isn't 730 days when a leap year is involved, and `0.5` years has no exact meaning. Validity should be in the unit the regulator uses (calendar months, or "end of month"), with the rule named in code, like `CalendarMonths(24)` or `EndOfMonth(24)` from task 2.

4. **`age < days_valid` is an off-by-one decision nobody made.** Is the last day included? With `<`, a licence completed on 10 March is invalid on the 730th day. Whatever the answer, it has to come from the business, be written in the code's documentation, and have a test for exactly that day.

5. **The date is stored as a `String` and parsed with `unwrap()`.** A badly formatted date crashes the dashboard. Parse once, when the data arrives, into a `NaiveDate`, and reject bad input there with a proper error. Then `Licence` can only hold real dates.

6. **`as i64` on an `f64`** silently truncates (and turns `NaN` into 0). If the conversion stays, it needs a reason. With fix 3 it disappears.

7. **"Checked it against a few licences"** means the boundaries probably weren't tested: the last valid day, the first invalid day, 29 February, a licence completed today, and the hours just after midnight in Nairobi. Ask for a table-driven test with those rows.

A rewrite that fixes 1–5:

```rust
pub struct Licence {
    pub completed_on: NaiveDate,
    pub validity: CalendarMonths,
}

pub fn is_valid(licence: &Licence, today: NaiveDate) -> bool {
    is_valid_on(&licence.validity, licence.completed_on, today)
}
```
