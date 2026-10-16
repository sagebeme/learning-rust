use ownership_memory::t03_dedupe_codes::dedupe_codes;

fn codes(list: &[&str]) -> Vec<String> {
    list.iter().map(|code| code.to_string()).collect()
}

#[test]
fn keeps_the_first_of_each_code_in_order() {
    let mut list = codes(&["FIRE", "DG", "FIRE", "AVSEC", "DG"]);
    dedupe_codes(&mut list);
    assert_eq!(list, ["FIRE", "DG", "AVSEC"]);
}

#[test]
fn a_list_without_repeats_is_unchanged() {
    let mut list = codes(&["RAMP", "FORKLIFT"]);
    dedupe_codes(&mut list);
    assert_eq!(list, ["RAMP", "FORKLIFT"]);
}

#[test]
fn codes_are_case_sensitive() {
    let mut list = codes(&["DG", "dg"]);
    dedupe_codes(&mut list);
    assert_eq!(list, ["DG", "dg"]);
}

#[test]
fn an_empty_list_stays_empty() {
    let mut list: Vec<String> = Vec::new();
    dedupe_codes(&mut list);
    assert!(list.is_empty());
}
