use functions_modules::t00_station_code::is_station_code;

#[test]
fn accepts_three_upper_case_letters() {
    assert!(is_station_code("NBO"));
    assert!(is_station_code("MBA"));
}

#[test]
fn rejects_the_wrong_length() {
    assert!(!is_station_code(""));
    assert!(!is_station_code("NB"));
    assert!(!is_station_code("NBOX"));
}

#[test]
fn rejects_lower_case_digits_and_symbols() {
    assert!(!is_station_code("nbo"));
    assert!(!is_station_code("N8O"));
    assert!(!is_station_code("NB-"));
}

#[test]
fn rejects_letters_outside_english_even_if_they_look_right() {
    // "ÉBO" is three characters but four bytes, and É isn't an English letter.
    assert!(!is_station_code("ÉBO"));
}
