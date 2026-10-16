use collections_iterators::t02_chained_map::{ChainedMap, djb2};

#[test]
fn djb2_matches_the_alx_values() {
    assert_eq!(djb2(""), 5381);
    assert_eq!(djb2("cisfun"), 6_953_392_314_605);
}

#[test]
fn stores_and_finds_values() {
    let mut map = ChainedMap::new(16);
    assert_eq!(map.insert("48213", "Amina"), None);
    assert_eq!(map.insert("10007", "Kip"), None);
    assert_eq!(map.get("48213"), Some("Amina"));
    assert_eq!(map.get("10007"), Some("Kip"));
    assert_eq!(map.get("99999"), None);
    assert_eq!(map.len(), 2);
}

#[test]
fn replacing_a_value_returns_the_old_one_and_keeps_the_count() {
    let mut map = ChainedMap::new(16);
    map.insert("48213", "Amina");
    assert_eq!(
        map.insert("48213", "Amina Wanjiru"),
        Some("Amina".to_string())
    );
    assert_eq!(map.get("48213"), Some("Amina Wanjiru"));
    assert_eq!(map.len(), 1);
}

// With one bucket, every key collides. This is the case your ALX hash_table_set got wrong: it
// looked in the *next bucket* instead of along the chain, so an existing key was added again.
#[test]
fn collisions_update_the_right_key_in_the_chain() {
    let mut map = ChainedMap::new(1);
    map.insert("hetairas", "Jennie");
    map.insert("mentioner", "Jennie");
    map.insert("hetairas", "Jennie Z Chu");

    assert_eq!(map.len(), 2);
    assert_eq!(map.get("hetairas"), Some("Jennie Z Chu"));
    assert_eq!(map.get("mentioner"), Some("Jennie"));
}

#[test]
fn zero_buckets_still_works() {
    let mut map = ChainedMap::new(0);
    map.insert("a", "1");
    assert_eq!(map.get("a"), Some("1"));
}

#[test]
fn a_new_map_is_empty() {
    let map = ChainedMap::new(8);
    assert!(map.is_empty());
    assert_eq!(map.get("anything"), None);
}
