use std::fs;

use errors_files_cli::model::{ParseError, RosterError};
use errors_files_cli::t02_load_roster::load_roster;

#[test]
fn loads_a_roster_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("staff.csv");
    fs::write(&path, "staff_number,name,station\n48213,Amina,NBO\n").unwrap();

    let staff = load_roster(&path).unwrap();
    assert_eq!(staff.len(), 1);
    assert_eq!(staff[0].name, "Amina");
}

#[test]
fn a_missing_file_is_a_read_error_that_names_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing.csv");

    let error = load_roster(&path).unwrap_err();
    match &error {
        RosterError::Read {
            path: reported,
            source,
        } => {
            assert_eq!(reported, &path);
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
        }
        other => panic!("expected a read error, got {other:?}"),
    }
    assert!(
        error.to_string().contains("missing.csv"),
        "message: {error}"
    );
}

#[test]
fn a_bad_line_is_a_parse_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("staff.csv");
    fs::write(&path, "staff_number,name,station\n48213,,NBO\n").unwrap();

    match load_roster(&path).unwrap_err() {
        RosterError::Parse(ParseError::EmptyName { line }) => assert_eq!(line, 2),
        other => panic!("expected a parse error, got {other:?}"),
    }
}
