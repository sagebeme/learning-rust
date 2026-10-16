use collections_iterators::model::{Holding, Station};
use collections_iterators::t00_count_by_station::count_by_station;

#[test]
fn counts_per_station_in_a_fixed_order() {
    let holdings = [
        Holding::new(1, Station::Mombasa, "DG", 10),
        Holding::new(2, Station::Nairobi, "DG", 10),
        Holding::new(3, Station::Mombasa, "FIRE", 10),
        Holding::new(4, Station::Eldoret, "RAMP", 10),
    ];
    let counts: Vec<(Station, usize)> = count_by_station(&holdings).into_iter().collect();
    assert_eq!(
        counts,
        [
            (Station::Nairobi, 1),
            (Station::Mombasa, 2),
            (Station::Eldoret, 1)
        ]
    );
}

#[test]
fn no_holdings_no_counts() {
    assert!(count_by_station(&[]).is_empty());
}
