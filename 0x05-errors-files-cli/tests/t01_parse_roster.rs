use errors_files_cli::model::ParseError;
use errors_files_cli::t01_parse_roster::parse_roster;

const ROSTER: &str = "staff_number,name,station
48213,Amina Wanjiru,NBO

10007,Kip Ruto,MBA
";

#[test]
fn parses_every_staff_member_and_skips_the_header_and_blanks() {
    let staff = parse_roster(ROSTER).unwrap();
    let numbers: Vec<u32> = staff.iter().map(|s| s.staff_number).collect();
    assert_eq!(numbers, [48213, 10007]);
}

#[test]
fn counts_lines_including_the_header_and_blanks() {
    let text = "staff_number,name,station\n48213,Amina,NBO\n\n10007,Kip,LHR\n";
    assert_eq!(
        parse_roster(text),
        Err(ParseError::UnknownStation {
            line: 4,
            code: "LHR".to_string()
        })
    );
}

#[test]
fn stops_at_the_first_bad_line() {
    let text = "header\nbad line\nx,Kip,LHR\n";
    assert_eq!(
        parse_roster(text),
        Err(ParseError::WrongFieldCount { line: 2, found: 1 })
    );
}

#[test]
fn a_header_alone_is_an_empty_roster() {
    assert_eq!(parse_roster("staff_number,name,station\n"), Ok(vec![]));
    assert_eq!(parse_roster(""), Ok(vec![]));
}

#[test]
fn handles_windows_line_endings() {
    let text = "staff_number,name,station\r\n48213,Amina,NBO\r\n";
    assert_eq!(parse_roster(text).unwrap()[0].station, "NBO");
}
