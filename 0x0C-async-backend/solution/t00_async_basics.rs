//! Task 0: two async building blocks.
//!
//! - `with_timeout(future, limit)`: wait for `future`, but give up after `limit` with
//!   `Err(TimedOut)`. Use `tokio::time::timeout`.
//! - `count_all(stations, count)`: call `count(station)` for every station **at the same time**
//!   (spawn each on a `tokio::task::JoinSet`), wait for them all, and return
//!   `(station, count)` pairs sorted by station. If a task panics, let the panic continue
//!   (`std::panic::resume_unwind`).

use std::future::Future;
use std::time::Duration;

use tokio::task::JoinSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimedOut;

pub async fn with_timeout<F: Future>(future: F, limit: Duration) -> Result<F::Output, TimedOut> {
    tokio::time::timeout(limit, future)
        .await
        .map_err(|_| TimedOut)
}

pub async fn count_all<F, Fut>(stations: Vec<String>, count: F) -> Vec<(String, usize)>
where
    F: Fn(String) -> Fut,
    Fut: Future<Output = usize> + Send + 'static,
{
    let mut tasks = JoinSet::new();
    for station in stations {
        let counting = count(station.clone());
        tasks.spawn(async move { (station, counting.await) });
    }

    let mut results = Vec::new();
    while let Some(finished) = tasks.join_next().await {
        match finished {
            Ok(result) => results.push(result),
            Err(error) if error.is_panic() => std::panic::resume_unwind(error.into_panic()),
            // Only possible if the task was cancelled, which this function never does.
            Err(_) => {}
        }
    }
    // Tasks finish in whatever order they finish; sort so the answer never depends on it.
    results.sort();
    results
}
