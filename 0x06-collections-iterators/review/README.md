# Review: "Staff lookup cache and station report"

A teammate opens this PR: *"Adds a small cache for staff names, and the per-station report the training manager asked for. All tests pass."* Find the bugs. Some of them pass every test the author wrote.

```rust
use std::collections::HashMap;

pub struct NameCache {
    buckets: Vec<Vec<(u32, String)>>,
}

impl NameCache {
    pub fn set(&mut self, staff_number: u32, name: String) {
        let mut index = staff_number as usize % self.buckets.len();
        // update the name if we already have this person
        while index < self.buckets.len() && !self.buckets[index].is_empty() {
            if self.buckets[index][0].0 == staff_number {
                self.buckets[index][0].1 = name;
                return;
            }
            index += 1;
        }
        let home = staff_number as usize % self.buckets.len();
        self.buckets[home].push((staff_number, name));
    }
}

pub fn station_report(holdings: &[(String, String)]) -> String {
    // (station, qualification) pairs → "NBO: 12\nMBA: 4\n..."
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (station, _) in holdings {
        *counts.entry(station.clone()).or_insert(0) += 1;
    }
    let mut report = String::new();
    for (station, count) in &counts {
        report.push_str(&format!("{station}: {count}\n"));
    }
    report
}
```
