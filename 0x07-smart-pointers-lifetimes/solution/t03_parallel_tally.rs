//! Task 3: count expired qualifications at every station at once, with threads.
//!
//! Each inner list is one station's "days left" values. Start **one thread per station**, have
//! each count its negative values, and add them into one shared total: an `Arc<Mutex<usize>>`.
//! Return the total once every thread has finished.

use std::sync::{Arc, Mutex};
use std::thread;

pub fn count_expired_in_parallel(stations: Vec<Vec<i64>>) -> usize {
    let total = Arc::new(Mutex::new(0));
    let mut workers = Vec::new();

    for days_left in stations {
        let total = Arc::clone(&total);
        workers.push(thread::spawn(move || {
            // Count first, outside the lock, so the lock is held for as short a time as possible.
            let expired = days_left.iter().filter(|&&days| days < 0).count();
            // A poisoned lock means another thread panicked mid-update; the count is still usable.
            let mut total = total
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            *total += expired;
        }));
    }

    for worker in workers {
        if let Err(panic) = worker.join() {
            std::panic::resume_unwind(panic);
        }
    }

    let total = total
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *total
}
