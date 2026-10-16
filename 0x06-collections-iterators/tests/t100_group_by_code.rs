use collections_iterators::model::{Holding, Station};
use collections_iterators::t100_group_by_code::group_by_code;

#[test]
fn groups_sorted_staff_numbers_by_code() {
    let holdings = [
        Holding::new(30, Station::Nairobi, "DG", 1),
        Holding::new(10, Station::Nairobi, "FIRE", 1),
        Holding::new(20, Station::Kisumu, "DG", 1),
        Holding::new(30, Station::Kisumu, "DG", 1),
    ];
    let groups = group_by_code(&holdings);
    let as_pairs: Vec<(&str, Vec<u32>)> = groups.into_iter().collect();
    assert_eq!(as_pairs, [("DG", vec![20, 30]), ("FIRE", vec![10])]);
}

#[test]
fn nothing_to_group() {
    assert!(group_by_code(&[]).is_empty());
}
