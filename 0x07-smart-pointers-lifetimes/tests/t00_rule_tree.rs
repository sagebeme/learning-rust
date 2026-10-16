use smart_pointers_lifetimes::t00_rule_tree::Rule;

fn ramp_supervisor() -> Rule {
    // FIRE, and also DG or AVSEC
    Rule::all(
        Rule::has("FIRE"),
        Rule::any(Rule::has("DG"), Rule::has("AVSEC")),
    )
}

#[test]
fn a_single_qualification() {
    assert!(Rule::has("FIRE").is_met(&["FIRE", "DG"]));
    assert!(!Rule::has("FIRE").is_met(&["DG"]));
}

#[test]
fn all_needs_both_sides() {
    let rule = ramp_supervisor();
    assert!(rule.is_met(&["FIRE", "DG"]));
    assert!(rule.is_met(&["AVSEC", "FIRE"]));
    assert!(!rule.is_met(&["DG", "AVSEC"]));
    assert!(!rule.is_met(&["FIRE"]));
}

#[test]
fn not_inverts() {
    // Must hold FIRE and must NOT still be a trainee.
    let rule = Rule::all(Rule::has("FIRE"), Rule::negate(Rule::has("TRAINEE")));
    assert!(rule.is_met(&["FIRE"]));
    assert!(!rule.is_met(&["FIRE", "TRAINEE"]));
}

#[test]
fn counts_conditions() {
    assert_eq!(Rule::has("FIRE").conditions(), 1);
    assert_eq!(ramp_supervisor().conditions(), 3);
    assert_eq!(Rule::negate(ramp_supervisor()).conditions(), 3);
}

#[test]
fn works_on_a_deep_rule() {
    let mut rule = Rule::has("Q0");
    for i in 1..1_000 {
        rule = Rule::any(rule, Rule::has(&format!("Q{i}")));
    }
    assert_eq!(rule.conditions(), 1_000);
    assert!(rule.is_met(&["Q0"]));
}
