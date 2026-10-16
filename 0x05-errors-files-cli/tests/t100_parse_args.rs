use std::path::PathBuf;

use errors_files_cli::model::{Command, UsageError};
use errors_files_cli::t100_parse_args::parse_args;

#[test]
fn check_a_file() {
    assert_eq!(
        parse_args(&["check", "staff.csv"]),
        Ok(Command::Check {
            file: PathBuf::from("staff.csv")
        })
    );
}

#[test]
fn count_with_and_without_a_station() {
    assert_eq!(
        parse_args(&["count", "staff.csv"]),
        Ok(Command::Count {
            file: PathBuf::from("staff.csv"),
            station: None
        })
    );
    assert_eq!(
        parse_args(&["count", "staff.csv", "--station", "NBO"]),
        Ok(Command::Count {
            file: PathBuf::from("staff.csv"),
            station: Some("NBO".to_string())
        })
    );
}

#[test]
fn usage_errors() {
    assert_eq!(parse_args(&[]), Err(UsageError::NoCommand));
    assert_eq!(
        parse_args(&["delete", "staff.csv"]),
        Err(UsageError::UnknownCommand("delete".to_string()))
    );
    assert_eq!(
        parse_args(&["check"]),
        Err(UsageError::MissingFile {
            command: "check".to_string()
        })
    );
    assert_eq!(
        parse_args(&["count", "staff.csv", "--station"]),
        Err(UsageError::MissingStation)
    );
    assert_eq!(
        parse_args(&["check", "staff.csv", "--station", "NBO"]),
        Err(UsageError::Unexpected("--station".to_string()))
    );
    assert_eq!(
        parse_args(&["count", "staff.csv", "--station", "NBO", "extra"]),
        Err(UsageError::Unexpected("extra".to_string()))
    );
    assert_eq!(
        parse_args(&["count", "staff.csv", "NBO"]),
        Err(UsageError::Unexpected("NBO".to_string()))
    );
}
