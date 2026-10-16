# Review: "Load station config"

A teammate opens this PR: *"Loads each station's settings when the app starts. Works on my machine."* Write your review: what's wrong, why it matters for a station that's offline and on generator power, and what to do instead.

```rust
use std::error::Error;
use std::fs;

pub struct StationConfig {
    pub code: String,
    pub alert_days: u32,
}

pub fn load_config(path: &str) -> StationConfig {
    let text = fs::read_to_string(path).unwrap();
    let mut lines = text.lines();
    let code = lines.next().unwrap().to_string();
    let alert_days = lines.next().unwrap().parse().expect("bad number");
    StationConfig { code, alert_days }
}

pub fn save_config(path: &str, config: &StationConfig) -> Result<(), Box<dyn Error>> {
    let _ = fs::write(path, format!("{}\n{}\n", config.code, config.alert_days));
    Ok(())
}

pub fn alert_days_or_default(path: &str) -> u32 {
    match std::panic::catch_unwind(|| load_config(path)) {
        Ok(config) => config.alert_days,
        Err(_) => 30,
    }
}
```
