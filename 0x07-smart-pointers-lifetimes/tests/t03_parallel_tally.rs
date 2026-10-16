use smart_pointers_lifetimes::t03_parallel_tally::count_expired_in_parallel;

#[test]
fn adds_up_every_station() {
    let stations = vec![vec![10, -1, -5], vec![0, 30], vec![-2], vec![]];
    assert_eq!(count_expired_in_parallel(stations), 3);
}

#[test]
fn no_stations_means_nothing_expired() {
    assert_eq!(count_expired_in_parallel(vec![]), 0);
}

#[test]
fn many_stations_at_once_give_the_right_total() {
    // 50 stations with 1,000 values each, 500 of them negative: a race would lose some updates.
    let stations: Vec<Vec<i64>> = (0..50)
        .map(|_| {
            (0..1_000)
                .map(|n| if n % 2 == 0 { -1 } else { 1 })
                .collect()
        })
        .collect();
    assert_eq!(count_expired_in_parallel(stations), 25_000);
}
