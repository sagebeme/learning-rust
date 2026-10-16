//! Task 0: two async building blocks.
//!
//! - `with_timeout(future, limit)`: wait for `future`, but give up after `limit` with
//!   `Err(TimedOut)`. Use `tokio::time::timeout`.
//! - `count_all(stations, count)`: call `count(station)` for every station **at the same time**
//!   (spawn each on a `tokio::task::JoinSet`), wait for them all, and return
//!   `(station, count)` pairs sorted by station. If a task panics, let the panic continue
//!   (`std::panic::resume_unwind`).

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use std::future::Future;
use std::time::Duration;

use tokio::task::JoinSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimedOut;

pub async fn with_timeout<F: Future>(future: F, limit: Duration) -> Result<F::Output, TimedOut> {
    todo!()
}

pub async fn count_all<F, Fut>(stations: Vec<String>, count: F) -> Vec<(String, usize)>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = usize> + Send + 'static,
{
    todo!()
}
