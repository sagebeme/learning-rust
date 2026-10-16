use ownership_memory::t02_split_name::split_name;

#[test]
fn splits_first_name_from_the_rest() {
    assert_eq!(
        split_name("Amina Wanjiru Otieno"),
        ("Amina", "Wanjiru Otieno")
    );
}

#[test]
fn ignores_spaces_at_the_ends_and_after_the_first_name() {
    assert_eq!(split_name("  Kip   Ruto  "), ("Kip", "Ruto"));
}

#[test]
fn one_name_has_no_rest() {
    assert_eq!(split_name("Kip"), ("Kip", ""));
}

#[test]
fn an_empty_name() {
    assert_eq!(split_name(""), ("", ""));
    assert_eq!(split_name("   "), ("", ""));
}

#[test]
fn both_parts_point_into_the_original_text() {
    let full = String::from("Zawadi Njeri");
    let (first, rest) = split_name(&full);
    let range = full.as_ptr() as usize..full.as_ptr() as usize + full.len();
    assert!(range.contains(&(first.as_ptr() as usize)));
    assert!(range.contains(&(rest.as_ptr() as usize)));
}
