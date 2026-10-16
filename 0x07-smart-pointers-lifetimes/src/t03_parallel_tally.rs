//! Task 3: count expired qualifications at every station at once, with threads.
//!
//! Each inner list is one station's "days left" values. Start **one thread per station**, have
//! each count its negative values, and add them into one shared total: an `Arc<Mutex<usize>>`.
//! Return the total once every thread has finished.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::sync::{Arc, Mutex};
use std::thread;

pub fn count_expired_in_parallel(stations: Vec<Vec<i64>>) -> usize {
    todo!()
}
