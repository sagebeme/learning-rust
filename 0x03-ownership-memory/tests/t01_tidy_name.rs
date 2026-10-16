use ownership_memory::t01_tidy_name::tidy_name;

fn tidied(input: &str) -> String {
    let mut name = input.to_string();
    tidy_name(&mut name);
    name
}

#[test]
fn trims_both_ends() {
    assert_eq!(tidied("  Amina "), "Amina");
}

#[test]
fn squeezes_spaces_inside() {
    assert_eq!(tidied("Amina    Wanjiru   Otieno"), "Amina Wanjiru Otieno");
}

#[test]
fn leaves_a_tidy_name_alone() {
    assert_eq!(tidied("Kip Ruto"), "Kip Ruto");
}

#[test]
fn a_blank_name_becomes_empty() {
    assert_eq!(tidied("    "), "");
}

#[test]
fn changes_the_callers_string() {
    let mut name = String::from(" Zawadi  Njeri ");
    tidy_name(&mut name);
    tidy_name(&mut name);
    assert_eq!(name, "Zawadi Njeri");
}
