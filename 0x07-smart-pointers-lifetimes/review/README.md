# Review: "Speed up the expiry scan"

A teammate opens this PR: *"The nightly scan was slow, so I made it run on all four stations in parallel and cached the catalogue."* Review it. Focus on whether the smart pointers and locks are needed, and what each one costs.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Catalogue {
    pub codes: Rc<RefCell<Vec<String>>>,
}

impl Catalogue {
    pub fn contains(&self, code: &str) -> bool {
        self.codes.borrow().clone().contains(&code.to_string())
    }
}

pub fn scan(stations: Vec<Vec<(u32, i64)>>, notify: fn(u32)) -> usize {
    let expired = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for station in stations {
        let expired = expired.clone();
        handles.push(thread::spawn(move || {
            let mut list = expired.lock().unwrap();
            for (staff, days_left) in station.clone() {
                if days_left < 0 {
                    notify(staff); // sends an SMS; takes about a second
                    list.push(staff);
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let count = expired.lock().unwrap().len();
    count
}
```
