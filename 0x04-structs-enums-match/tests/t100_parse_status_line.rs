use structs_enums_match::t00_station::Station;
use structs_enums_match::t100_parse_status_line::parse_status_line;

#[test]
fn parses_a_good_line() {
    assert_eq!(
        parse_status_line("48213,NBO,12"),
        Some((48213, Station::Nairobi, 12))
    );
}

#[test]
fn allows_spaces_and_negative_days() {
    assert_eq!(
        parse_status_line(" 7 , mba , -3 "),
        Some((7, Station::Mombasa, -3))
    );
}

#[test]
fn rejects_malformed_lines() {
    for bad in [
        "",
        "48213,NBO",
        "48213,NBO,12,extra",
        "abc,NBO,12",
        "-5,NBO,12",
        "48213,LHR,12",
        "48213,NBO,12.5",
        "48213,NBO,",
    ] {
        assert_eq!(parse_status_line(bad), None, "should reject {bad:?}");
    }
}
