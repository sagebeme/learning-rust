use collections_iterators::t03_find_staff::find_staff;

const STAFF: [u32; 7] = [1001, 1004, 1010, 2002, 3003, 4040, 9999];

#[test]
fn finds_every_staff_number() {
    for (index, number) in STAFF.iter().enumerate() {
        assert_eq!(
            find_staff(&STAFF, *number),
            Some(index),
            "looking for {number}"
        );
    }
}

#[test]
fn a_number_between_entries_is_not_found() {
    assert_eq!(find_staff(&STAFF, 1005), None);
}

// The ALX bug: with unsigned indexes, "right = middle - 1" went below zero when the target was
// smaller than the first element.
#[test]
fn smaller_than_the_first_does_not_panic() {
    assert_eq!(find_staff(&STAFF, 0), None);
    assert_eq!(find_staff(&STAFF, 1000), None);
}

#[test]
fn bigger_than_the_last_does_not_panic() {
    assert_eq!(find_staff(&STAFF, 10_000), None);
    assert_eq!(find_staff(&STAFF, u32::MAX), None);
}

#[test]
fn empty_and_single_element_lists() {
    assert_eq!(find_staff(&[], 5), None);
    assert_eq!(find_staff(&[5], 5), Some(0));
    assert_eq!(find_staff(&[5], 4), None);
    assert_eq!(find_staff(&[5], 6), None);
}

#[test]
fn works_on_a_big_list() {
    let staff: Vec<u32> = (0..10_000).map(|n| n * 3).collect();
    assert_eq!(find_staff(&staff, 29_997), Some(9_999));
    assert_eq!(find_staff(&staff, 29_998), None);
}
