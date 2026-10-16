use hello_cargo::t00_welcome::welcome;

#[test]
fn welcomes_by_name() {
    assert_eq!(welcome("Amina"), "Karibu, Amina! Welcome to TWA.");
}

#[test]
fn works_for_any_name() {
    assert_eq!(welcome("Kip"), "Karibu, Kip! Welcome to TWA.");
}
