use ownership_memory::t00_longest_name::longest_name;

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|name| name.to_string()).collect()
}

#[test]
fn finds_the_longest() {
    let staff = names(&["Kip", "Zawadi", "Amina"]);
    assert_eq!(longest_name(&staff), Some("Zawadi"));
}

#[test]
fn the_first_one_wins_a_tie() {
    let staff = names(&["Amina", "Njeri", "Kip"]);
    assert_eq!(longest_name(&staff), Some("Amina"));
}

#[test]
fn counts_characters_not_bytes() {
    // "Émilie" is 6 characters but 7 bytes; "Wanjiru" is 7 characters.
    let staff = names(&["Émilie", "Wanjiru"]);
    assert_eq!(longest_name(&staff), Some("Wanjiru"));
}

#[test]
fn returns_a_borrow_not_a_copy() {
    let staff = names(&["Kip", "Zawadi"]);
    let longest = longest_name(&staff).unwrap();
    // Same memory address as the String inside the list: nothing was copied.
    assert_eq!(longest.as_ptr(), staff[1].as_ptr());
}

#[test]
fn an_empty_list_has_no_longest_name() {
    assert_eq!(longest_name(&[]), None);
}
