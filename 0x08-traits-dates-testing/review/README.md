# Review: "Expiry check for the dashboard"

A teammate opens this PR: *"Adds the function the dashboard uses to show whether a licence is still valid. Checked it against a few licences, all correct."* Review it.

```rust
use chrono::Utc;

pub struct Licence {
    pub completed_on: String, // "2025-03-10"
    pub valid_years: f64,     // 2.0 for two years
}

pub fn is_valid(licence: &Licence) -> bool {
    let completed = chrono::NaiveDate::parse_from_str(&licence.completed_on, "%Y-%m-%d").unwrap();
    let today = Utc::now().date_naive();
    let days_valid = (licence.valid_years * 365.0) as i64;
    let age = (today - completed).num_days();
    age < days_valid
}
```
