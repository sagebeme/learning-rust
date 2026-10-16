# Review: "Add qualification records"

A teammate opens this PR: *"Adds the record we'll store for each staff member's qualification. Tested by hand, works."* Write your review comments. For each one: what's wrong, why it matters for TWA, and what to do instead.

```rust
pub struct QualificationRecord {
    pub staff_number: String,
    pub station: String,       // "NBO", "MBA", "KIS" or "EDL"
    pub qualification: String,
    pub is_valid: bool,
    pub is_expired: bool,
    pub in_grace: bool,
    pub days_left: i64,        // ignore this if is_expired is true
    pub grace_days_used: i64,  // only set when in_grace is true
}

pub fn can_work(record: &QualificationRecord) -> bool {
    if record.is_valid {
        return true;
    }
    if record.in_grace && record.grace_days_used < 14 {
        return true;
    }
    false
}

pub fn station_name(record: &QualificationRecord) -> &str {
    if record.station == "NBO" {
        "Nairobi"
    } else if record.station == "MBA" {
        "Mombasa"
    } else if record.station == "KIS" {
        "Kisumu"
    } else {
        "Eldoret"
    }
}
```
