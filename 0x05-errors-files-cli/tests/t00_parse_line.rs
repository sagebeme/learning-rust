use errors_files_cli::model::{ParseError, StaffRecord};
use errors_files_cli::t00_parse_line::parse_staff_line;

#[test]
fn parses_a_good_line() {
    assert_eq!(
        parse_staff_line(2, " 48213 , Amina Wanjiru , NBO "),
        Ok(StaffRecord {
            staff_number: 48213,
            name: "Amina Wanjiru".to_string(),
            station: "NBO".to_string(),
        })
    );
}

#[test]
fn reports_the_wrong_number_of_fields() {
    assert_eq!(
        parse_staff_line(5, "48213,Amina"),
        Err(ParseError::WrongFieldCount { line: 5, found: 2 })
    );
    assert_eq!(
        parse_staff_line(6, "1,Amina,NBO,extra"),
        Err(ParseError::WrongFieldCount { line: 6, found: 4 })
    );
}

#[test]
fn reports_a_bad_staff_number_with_its_text() {
    assert_eq!(
        parse_staff_line(3, " 48a13 ,Amina,NBO"),
        Err(ParseError::BadStaffNumber {
            line: 3,
            value: "48a13".to_string()
        })
    );
}

#[test]
fn reports_an_empty_name() {
    assert_eq!(
        parse_staff_line(4, "48213,  ,NBO"),
        Err(ParseError::EmptyName { line: 4 })
    );
}

#[test]
fn reports_an_unknown_station() {
    assert_eq!(
        parse_staff_line(7, "48213,Amina,nbo"),
        Err(ParseError::UnknownStation {
            line: 7,
            code: "nbo".to_string()
        })
    );
}

#[test]
fn checks_problems_in_order() {
    // Both the number and the station are wrong: the number is reported, because it comes first.
    assert!(matches!(
        parse_staff_line(1, "x,Amina,LHR"),
        Err(ParseError::BadStaffNumber { .. })
    ));
}

#[test]
fn error_messages_tell_a_person_where_to_look() {
    let error = parse_staff_line(9, "48213,Amina,LHR").unwrap_err();
    assert_eq!(error.to_string(), r#"line 9: unknown station "LHR""#);
}
