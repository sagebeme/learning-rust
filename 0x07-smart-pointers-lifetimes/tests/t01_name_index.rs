use smart_pointers_lifetimes::t01_name_index::NameIndex;

const NAMES: &str = "  Amina Wanjiru\nKip Ruto\n\nAmani Otieno\n   \nZawadi Njeri  \n";

#[test]
fn keeps_trimmed_names_and_skips_blanks() {
    let index = NameIndex::new(NAMES);
    assert_eq!(index.len(), 4);
    assert_eq!(
        index.starting_with(""),
        ["Amina Wanjiru", "Kip Ruto", "Amani Otieno", "Zawadi Njeri"]
    );
}

#[test]
fn finds_names_by_prefix() {
    let index = NameIndex::new(NAMES);
    assert_eq!(index.starting_with("Am"), ["Amina Wanjiru", "Amani Otieno"]);
    assert!(index.starting_with("X").is_empty());
}

#[test]
fn finds_the_longest() {
    let index = NameIndex::new(NAMES);
    // "Amina Wanjiru" is 13 characters; the others are 12 or fewer.
    assert_eq!(index.longest(), Some("Amina Wanjiru"));
    assert_eq!(NameIndex::new("").longest(), None);
}

#[test]
fn results_outlive_the_index() {
    let text = String::from("Kip Ruto\nAmina Wanjiru\n");
    let found;
    {
        let index = NameIndex::new(&text);
        found = index.starting_with("Am");
    } // the index is gone here...
    assert_eq!(found, ["Amina Wanjiru"]); // ...but the results borrow from `text`, which isn't
}

#[test]
fn borrows_instead_of_copying() {
    let text = String::from("Kip Ruto");
    let index = NameIndex::new(&text);
    assert_eq!(index.longest().unwrap().as_ptr(), text.as_ptr());
}
