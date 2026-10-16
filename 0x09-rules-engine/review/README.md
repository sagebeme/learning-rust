# Review: "Gap check v1"

A teammate opens this PR: *"First version of the gap check. Tested with Amina's transfer from Ramp to Cargo, which gives the right answer."* Review it as if it's going into the engine every TWA device will run.

```rust
use std::collections::HashMap;

pub fn gap_check(
    required: &[String],
    holdings: &HashMap<String, NaiveDate>, // code → completed_on
    catalogue: &HashMap<String, QualificationType>,
) -> HashMap<String, String> {
    let today = chrono::Local::now().date_naive();
    let mut report = HashMap::new();
    for code in required {
        let q = &catalogue[code];
        match holdings.get(code) {
            None => {
                report.insert(code.clone(), "missing".to_string());
            }
            Some(completed) => {
                let until = *completed + chrono::Duration::days(q.validity_months as i64 * 30);
                if today <= until {
                    report.insert(code.clone(), "ok".to_string());
                } else {
                    report.insert(code.clone(), "expired".to_string());
                }
            }
        }
    }
    report
}
```
