use collections_iterators::model::{Holding, Station};
use collections_iterators::t01_expiring_within::expiring_within;

fn sample() -> Vec<Holding> {
    vec![
        Holding::new(30, Station::Nairobi, "DG", 12),
        Holding::new(10, Station::Nairobi, "FIRE", 3),
        Holding::new(20, Station::Kisumu, "RAMP", 3),
        Holding::new(40, Station::Mombasa, "AVSEC", 45),
        Holding::new(50, Station::Mombasa, "DG", -2),
        Holding::new(60, Station::Eldoret, "FIRE", 0),
    ]
}

#[test]
fn soonest_first_then_by_staff_number() {
    let holdings = sample();
    let soon: Vec<(u32, i64)> = expiring_within(&holdings, 30)
        .iter()
        .map(|h| (h.staff_number, h.days_left))
        .collect();
    assert_eq!(soon, [(60, 0), (10, 3), (20, 3), (30, 12)]);
}

#[test]
fn the_window_is_inclusive() {
    let holdings = sample();
    assert_eq!(expiring_within(&holdings, 12).len(), 4);
    assert_eq!(expiring_within(&holdings, 11).len(), 3);
}

#[test]
fn leaves_out_expired_holdings() {
    let holdings = sample();
    assert!(
        expiring_within(&holdings, 1000)
            .iter()
            .all(|h| h.days_left >= 0)
    );
}

#[test]
fn returns_references_into_the_input() {
    let holdings = sample();
    let soon = expiring_within(&holdings, 0);
    assert!(std::ptr::eq(soon[0], &holdings[5]));
}
