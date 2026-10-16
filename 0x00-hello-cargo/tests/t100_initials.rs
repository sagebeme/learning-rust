use hello_cargo::t100_initials::initials;

#[test]
fn makes_initials() {
    assert_eq!(initials("Amina Wanjiru Otieno"), "A.W.O.");
}

#[test]
fn ignores_extra_spaces() {
    assert_eq!(initials("  Kip   Ruto "), "K.R.");
}

#[test]
fn upper_cases_lower_case_names() {
    assert_eq!(initials("zawadi njeri"), "Z.N.");
}

#[test]
fn handles_letters_outside_english() {
    assert_eq!(initials("Émile Ödegaard"), "É.Ö.");
}

#[test]
fn gives_nothing_for_an_empty_name() {
    assert_eq!(initials(""), "");
    assert_eq!(initials("   "), "");
}
