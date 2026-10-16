use ownership_memory::t04_badge_lines::badge_lines;

#[test]
fn makes_a_badge_line_per_name() {
    let names = vec!["Amina".to_string(), "Kip Ruto".to_string()];
    assert_eq!(badge_lines(names), ["BADGE: AMINA", "BADGE: KIP RUTO"]);
}

#[test]
fn no_names_no_badges() {
    assert!(badge_lines(Vec::new()).is_empty());
}
