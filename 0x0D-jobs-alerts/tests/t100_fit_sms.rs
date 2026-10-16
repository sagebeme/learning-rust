use jobs_alerts::t100_fit_sms::fit_sms;

fn units(text: &str) -> usize {
    text.encode_utf16().count()
}

#[test]
fn a_short_message_is_unchanged() {
    let message = "Your DG expires in 7 days. Book a refresher at the NBO training room.";
    assert_eq!(fit_sms(message), message);
}

#[test]
fn exactly_160_plain_characters_fits() {
    let message = "a".repeat(160);
    assert_eq!(fit_sms(&message), message);
}

#[test]
fn a_long_plain_message_is_cut_at_a_word_and_marked() {
    let message = "Your Dangerous Goods certificate expires in 7 days. ".repeat(4);
    let fitted = fit_sms(&message);
    assert!(fitted.len() <= 160, "{} characters", fitted.len());
    assert!(fitted.ends_with("..."));
    assert!(
        !fitted.ends_with(" ..."),
        "no space before the dots: {fitted:?}"
    );
    assert!(message.starts_with(fitted.trim_end_matches("...")));
}

#[test]
fn one_accented_letter_switches_to_the_70_unit_limit() {
    let message = format!(
        "Habari Zawadi, kozi ya usalama inaanza Jumatatu. Café ya NBO ina chai. {}",
        "x".repeat(10)
    );
    let fitted = fit_sms(&message);
    assert!(units(&fitted) <= 70, "{} units", units(&fitted));
    assert!(fitted.ends_with("..."));
}

#[test]
fn an_emoji_counts_as_two() {
    let message = format!("{}✈️", "a".repeat(68));
    // 68 + the plane (1 unit) + the variation selector (1 unit) = 70: fits exactly.
    assert_eq!(fit_sms(&message), message);
    let message = format!("{}😀", "a".repeat(69));
    // 69 + an emoji outside the basic plane (2 units) = 71: too long.
    assert!(units(&fit_sms(&message)) <= 70);
}

#[test]
fn a_message_with_no_spaces_is_cut_exactly() {
    let message = "x".repeat(200);
    let fitted = fit_sms(&message);
    assert_eq!(fitted, format!("{}...", "x".repeat(157)));
}
