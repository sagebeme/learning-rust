//! Every test gets a fresh, empty database with the migrations applied (`#[sqlx::test]`), so
//! tests never see each other's data. They need a Postgres server: see the README.

// `#[sqlx::test]` wraps each test in its own function, which hides it from clippy's
// "unwrap is fine in tests" setting, so allow it here explicitly.
#![allow(clippy::unwrap_used)]

use chrono::NaiveDate;
use databases::model::{DbError, NewEmployee};
use databases::t00_insert_employee::insert_employee;
use databases::t01_record_holding::record_holding;
use databases::t02_expiring_between::expiring_between;
use databases::t03_transfer_employee::transfer_employee;
use databases::t100_import_employees::import_employees;
use sqlx::PgPool;

fn d(y: i32, m: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, day).expect("a real date")
}

async fn department_of(pool: &PgPool, staff_number: i32) -> String {
    sqlx::query_scalar("SELECT department FROM employees WHERE staff_number = $1")
        .bind(staff_number)
        .fetch_one(pool)
        .await
        .expect("the employee exists")
}

async fn count(pool: &PgPool, table: &str) -> i64 {
    // The table name comes from the test itself, never from users, so formatting it in is safe.
    sqlx::query_scalar(&format!("SELECT count(*) FROM {table}"))
        .fetch_one(pool)
        .await
        .expect("count works")
}

async fn seed(pool: &PgPool) {
    for employee in [
        NewEmployee::new(48213, "Amina Wanjiru", "NBO", "Ramp"),
        NewEmployee::new(10007, "Kip Ruto", "MBA", "Cargo"),
        NewEmployee::new(20020, "Zawadi Njeri", "NBO", "Cargo"),
    ] {
        insert_employee(pool, &employee)
            .await
            .expect("seed employee");
    }
}

// ---- task 0 ----

#[sqlx::test(migrations = "./migrations")]
async fn t00_inserts_an_employee(pool: PgPool) {
    insert_employee(
        &pool,
        &NewEmployee::new(48213, "Amina Wanjiru", "NBO", "Ramp"),
    )
    .await
    .unwrap();
    assert_eq!(department_of(&pool, 48213).await, "Ramp");
}

#[sqlx::test(migrations = "./migrations")]
async fn t00_a_taken_staff_number_is_a_duplicate(pool: PgPool) {
    seed(&pool).await;
    let result = insert_employee(
        &pool,
        &NewEmployee::new(48213, "Someone Else", "MBA", "Cargo"),
    )
    .await;
    assert!(
        matches!(result, Err(DbError::DuplicateStaffNumber(48213))),
        "{result:?}"
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn t00_an_unknown_station_is_reported(pool: PgPool) {
    let result = insert_employee(&pool, &NewEmployee::new(1, "Kip Ruto", "LHR", "Cargo")).await;
    assert!(
        matches!(&result, Err(DbError::UnknownStation(code)) if code == "LHR"),
        "{result:?}"
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn t00_the_database_refuses_a_blank_name(pool: PgPool) {
    let result = insert_employee(&pool, &NewEmployee::new(1, "   ", "NBO", "Cargo")).await;
    assert!(matches!(result, Err(DbError::Sqlx(_))), "{result:?}");
}

// ---- task 1 ----

#[sqlx::test(migrations = "./migrations")]
async fn t01_records_and_renews_a_holding(pool: PgPool) {
    seed(&pool).await;
    assert!(
        record_holding(&pool, 48213, "DG", d(2024, 2, 1), d(2026, 2, 1))
            .await
            .unwrap()
    );
    assert!(
        record_holding(&pool, 48213, "DG", d(2026, 2, 1), d(2028, 2, 1))
            .await
            .unwrap()
    );
    let rows = expiring_between(&pool, d(2000, 1, 1), d(2100, 1, 1), None)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].valid_until, d(2028, 2, 1));
}

#[sqlx::test(migrations = "./migrations")]
async fn t01_an_older_record_never_replaces_a_newer_one(pool: PgPool) {
    seed(&pool).await;
    record_holding(&pool, 48213, "DG", d(2026, 2, 1), d(2028, 2, 1))
        .await
        .unwrap();
    assert!(
        !record_holding(&pool, 48213, "DG", d(2024, 2, 1), d(2026, 2, 1))
            .await
            .unwrap()
    );
    let rows = expiring_between(&pool, d(2000, 1, 1), d(2100, 1, 1), None)
        .await
        .unwrap();
    assert_eq!(rows[0].valid_until, d(2028, 2, 1));
}

#[sqlx::test(migrations = "./migrations")]
async fn t01_a_holding_for_an_unknown_employee_is_not_found(pool: PgPool) {
    let result = record_holding(&pool, 99999, "DG", d(2026, 2, 1), d(2028, 2, 1)).await;
    assert!(
        matches!(result, Err(DbError::NotFound(99999))),
        "{result:?}"
    );
}

// ---- task 2 ----

#[sqlx::test(migrations = "./migrations")]
async fn t02_lists_what_expires_in_order(pool: PgPool) {
    seed(&pool).await;
    record_holding(&pool, 48213, "FIRE", d(2025, 11, 1), d(2026, 11, 1))
        .await
        .unwrap();
    record_holding(&pool, 20020, "DG", d(2024, 10, 20), d(2026, 10, 20))
        .await
        .unwrap();
    record_holding(&pool, 10007, "DG", d(2024, 11, 1), d(2026, 11, 1))
        .await
        .unwrap();
    record_holding(&pool, 48213, "AIRSIDE", d(2024, 11, 1), d(2026, 11, 1))
        .await
        .unwrap();
    record_holding(&pool, 10007, "FIRE", d(2026, 1, 1), d(2027, 1, 1))
        .await
        .unwrap();

    let rows = expiring_between(&pool, d(2026, 10, 16), d(2026, 11, 15), None)
        .await
        .unwrap();
    let keys: Vec<(i32, &str)> = rows
        .iter()
        .map(|r| (r.staff_number, r.code.as_str()))
        .collect();
    assert_eq!(
        keys,
        [
            (20020, "DG"),
            (10007, "DG"),
            (48213, "AIRSIDE"),
            (48213, "FIRE")
        ]
    );
    assert_eq!(rows[0].name, "Zawadi Njeri");
    assert_eq!(rows[0].station_code, "NBO");
}

#[sqlx::test(migrations = "./migrations")]
async fn t02_both_ends_of_the_range_are_included_and_a_station_filter_works(pool: PgPool) {
    seed(&pool).await;
    record_holding(&pool, 48213, "FIRE", d(2025, 10, 16), d(2026, 10, 16))
        .await
        .unwrap();
    record_holding(&pool, 10007, "FIRE", d(2025, 11, 15), d(2026, 11, 15))
        .await
        .unwrap();

    let all = expiring_between(&pool, d(2026, 10, 16), d(2026, 11, 15), None)
        .await
        .unwrap();
    assert_eq!(all.len(), 2);
    let mombasa = expiring_between(&pool, d(2026, 10, 16), d(2026, 11, 15), Some("MBA"))
        .await
        .unwrap();
    assert_eq!(mombasa.len(), 1);
    assert_eq!(mombasa[0].staff_number, 10007);
}

// ---- task 3 ----

#[sqlx::test(migrations = "./migrations")]
async fn t03_transfers_and_records_the_move(pool: PgPool) {
    seed(&pool).await;
    let from = transfer_employee(&pool, 48213, "Cargo", d(2026, 10, 16))
        .await
        .unwrap();
    assert_eq!(from, "Ramp");
    assert_eq!(department_of(&pool, 48213).await, "Cargo");
    assert_eq!(count(&pool, "transfers").await, 1);
}

#[sqlx::test(migrations = "./migrations")]
async fn t03_an_unknown_employee_is_not_found(pool: PgPool) {
    let result = transfer_employee(&pool, 99999, "Cargo", d(2026, 10, 16)).await;
    assert!(
        matches!(result, Err(DbError::NotFound(99999))),
        "{result:?}"
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn t03_a_failed_audit_row_undoes_the_department_change(pool: PgPool) {
    seed(&pool).await;
    // The transfers table refuses a blank department, so the last step fails...
    let result = transfer_employee(&pool, 48213, "  ", d(2026, 10, 16)).await;
    assert!(result.is_err());
    // ...and the department change from the step before must have been rolled back.
    assert_eq!(department_of(&pool, 48213).await, "Ramp");
    assert_eq!(count(&pool, "transfers").await, 0);
}

// ---- task 100 ----

#[sqlx::test(migrations = "./migrations")]
async fn t100_imports_a_whole_roster(pool: PgPool) {
    let roster = [
        NewEmployee::new(1, "Amina Wanjiru", "NBO", "Ramp"),
        NewEmployee::new(2, "Kip Ruto", "MBA", "Cargo"),
    ];
    assert_eq!(import_employees(&pool, &roster).await.unwrap(), 2);
    assert_eq!(count(&pool, "employees").await, 2);
}

#[sqlx::test(migrations = "./migrations")]
async fn t100_one_bad_row_saves_nothing_and_says_which(pool: PgPool) {
    let roster = [
        NewEmployee::new(1, "Amina Wanjiru", "NBO", "Ramp"),
        NewEmployee::new(2, "Kip Ruto", "MBA", "Cargo"),
        NewEmployee::new(3, "Zawadi Njeri", "LHR", "Cargo"),
    ];
    let result = import_employees(&pool, &roster).await;
    match result {
        Err(DbError::InRow { row, source }) => {
            assert_eq!(row, 3);
            assert!(matches!(*source, DbError::UnknownStation(_)));
        }
        other => panic!("expected an error in row 3, got {other:?}"),
    }
    assert_eq!(count(&pool, "employees").await, 0);
}
