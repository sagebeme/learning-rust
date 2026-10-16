use std::time::Duration;

use async_backend::t00_async_basics::{TimedOut, count_all, with_timeout};
use tokio::time::{Instant, sleep};

// `start_paused` freezes Tokio's clock and jumps it forward whenever every task is waiting, so
// these tests about seconds-long waits finish instantly.

#[tokio::test(start_paused = true)]
async fn returns_the_answer_when_it_arrives_in_time() {
    let answer = with_timeout(
        async {
            sleep(Duration::from_secs(2)).await;
            42
        },
        Duration::from_secs(5),
    )
    .await;
    assert_eq!(answer, Ok(42));
}

#[tokio::test(start_paused = true)]
async fn gives_up_after_the_limit() {
    let started = Instant::now();
    let answer = with_timeout(sleep(Duration::from_secs(60)), Duration::from_secs(5)).await;
    assert_eq!(answer, Err(TimedOut));
    assert_eq!(started.elapsed(), Duration::from_secs(5));
}

#[tokio::test(start_paused = true)]
async fn counts_every_station_at_the_same_time() {
    let stations: Vec<String> = ["NBO", "MBA", "KIS", "EDL"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let started = Instant::now();
    let counts = count_all(stations, |station| async move {
        // Nairobi's lookup takes 3 seconds; the others take 1.
        let delay = if station == "NBO" { 3 } else { 1 };
        sleep(Duration::from_secs(delay)).await;
        station.len() * 10
    })
    .await;

    assert_eq!(
        counts,
        [
            ("EDL".to_string(), 30),
            ("KIS".to_string(), 30),
            ("MBA".to_string(), 30),
            ("NBO".to_string(), 30),
        ]
    );
    // One after another would take 6 seconds; at the same time, it's the slowest one: 3.
    assert_eq!(started.elapsed(), Duration::from_secs(3));
}

#[tokio::test]
async fn no_stations_no_counts() {
    let counts = count_all(Vec::new(), |_| async { 1 }).await;
    assert!(counts.is_empty());
}
