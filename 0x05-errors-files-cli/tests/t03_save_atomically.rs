use std::fs;

use errors_files_cli::t03_save_atomically::save_atomically;

#[test]
fn writes_a_new_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("roster.csv");

    save_atomically(&path, "staff_number,name,station\n").unwrap();
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "staff_number,name,station\n"
    );
}

#[test]
fn replaces_an_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("roster.csv");
    fs::write(&path, "old contents").unwrap();

    save_atomically(&path, "new contents").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "new contents");
}

#[test]
fn leaves_no_temporary_file_behind() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("roster.csv");

    save_atomically(&path, "data").unwrap();
    let names: Vec<String> = fs::read_dir(dir.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, ["roster.csv"]);
}

#[test]
fn uses_the_temporary_name_in_the_same_folder() {
    // If the temporary file can't be created, the real file must be untouched. Blocking the
    // temporary name with a folder of the same name makes creating it fail on every OS.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("roster.csv");
    fs::write(&path, "original").unwrap();
    fs::create_dir(dir.path().join(".roster.csv.tmp")).unwrap();

    assert!(save_atomically(&path, "new").is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), "original");
}

#[test]
fn reports_a_folder_that_does_not_exist() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("no-such-folder").join("roster.csv");
    assert!(save_atomically(&path, "data").is_err());
}
