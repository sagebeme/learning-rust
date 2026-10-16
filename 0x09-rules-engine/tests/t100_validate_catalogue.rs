mod common;

use common::{catalogue_types, qualification};
use rules_engine::model::CatalogueError;
use rules_engine::t100_validate_catalogue::validate_catalogue;

#[test]
fn accepts_a_good_catalogue() {
    let catalogue = validate_catalogue(catalogue_types()).unwrap();
    assert_eq!(catalogue.types.len(), 5);
}

#[test]
fn rejects_a_duplicate_code() {
    let mut types = catalogue_types();
    types.push(qualification("FIRE", 6, 0, 0, None, &[], &[]));
    assert_eq!(
        validate_catalogue(types),
        Err(CatalogueError::DuplicateCode("FIRE".to_string()))
    );
}

#[test]
fn rejects_a_reference_to_a_code_that_does_not_exist() {
    let mut types = catalogue_types();
    types.push(qualification(
        "TUG",
        12,
        0,
        30,
        None,
        &["AIRSIDE", "TOWING"],
        &[],
    ));
    assert_eq!(
        validate_catalogue(types),
        Err(CatalogueError::UnknownReference {
            code: "TUG".to_string(),
            refers_to: "TOWING".to_string()
        })
    );
}

#[test]
fn checks_equivalents_too() {
    let types = vec![qualification("A", 12, 0, 0, None, &[], &["B"])];
    assert_eq!(
        validate_catalogue(types),
        Err(CatalogueError::UnknownReference {
            code: "A".to_string(),
            refers_to: "B".to_string()
        })
    );
}

#[test]
fn rejects_a_prerequisite_cycle_and_names_what_it_blocks() {
    let types = vec![
        qualification("A", 12, 0, 0, None, &["B"], &[]),
        qualification("B", 12, 0, 0, None, &["A"], &[]),
        qualification("C", 12, 0, 0, None, &["A"], &[]),
        qualification("D", 12, 0, 0, None, &[], &[]),
    ];
    assert_eq!(
        validate_catalogue(types),
        Err(CatalogueError::PrerequisiteCycle(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string()
        ]))
    );
}
