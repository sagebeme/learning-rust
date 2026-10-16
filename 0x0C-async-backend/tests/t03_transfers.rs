mod common;

use async_backend::app::routes;
use axum::http::StatusCode;
use common::{call, state};
use serde_json::json;

async fn check(body: serde_json::Value) -> common::Answer {
    call(
        routes(state()),
        "POST",
        "/transfers/check",
        Some(body),
        None,
    )
    .await
}

#[tokio::test]
async fn lists_what_is_missing() {
    let answer =
        check(json!({ "staff_number": 20020, "department": "Cargo", "role": "Handler" })).await;
    assert_eq!(answer.status, StatusCode::OK);
    assert_eq!(
        answer.json(),
        json!({ "ready": false, "missing": ["DG", "FORKLIFT"] })
    );
}

#[tokio::test]
async fn ready_when_nothing_is_missing() {
    let answer =
        check(json!({ "staff_number": 10007, "department": "Cargo", "role": "Handler" })).await;
    assert_eq!(answer.json(), json!({ "ready": true, "missing": [] }));
}

#[tokio::test]
async fn someone_with_nothing_misses_everything_in_order() {
    let answer =
        check(json!({ "staff_number": 30303, "department": "Ramp", "role": "Agent" })).await;
    assert_eq!(
        answer.json(),
        json!({ "ready": false, "missing": ["AIRSIDE", "FIRE"] })
    );
}

#[tokio::test]
async fn an_unknown_employee_is_404() {
    let answer =
        check(json!({ "staff_number": 99999, "department": "Cargo", "role": "Handler" })).await;
    assert_eq!(answer.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_unknown_role_is_422() {
    let answer =
        check(json!({ "staff_number": 10007, "department": "Cargo", "role": "Pilot" })).await;
    assert_eq!(answer.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        answer.json()["message"],
        "no requirements for Pilot in Cargo"
    );
}
