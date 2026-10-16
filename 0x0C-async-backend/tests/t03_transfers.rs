//! Amina's transfer, asked from the server.

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
async fn amina_is_not_ready_for_cargo_and_the_report_says_why() {
    let answer = check(json!({
        "staff_number": 48213, "department": "Cargo", "role": "Handler", "on": "2026-10-19"
    }))
    .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.text);
    assert_eq!(
        answer.json(),
        json!({
            "ready": false,
            "report": {
                "department": "Cargo",
                "role": "Handler",
                "lines": [
                    { "code": "AIRSIDE", "state": { "state": "met", "valid_until": "2027-03-10" }, "satisfied_by": "AIRSIDE" },
                    { "code": "DG", "state": { "state": "expiring_soon", "valid_until": "2026-11-01", "days_left": 13 }, "satisfied_by": "DG-ICAO" },
                    { "code": "FORKLIFT", "state": { "state": "missing" }, "satisfied_by": null }
                ]
            }
        })
    );
}

#[tokio::test]
async fn kip_is_ready() {
    let answer = check(json!({
        "staff_number": 10007, "department": "Cargo", "role": "Handler", "on": "2026-10-19"
    }))
    .await;
    assert_eq!(answer.json()["ready"], true);
}

#[tokio::test]
async fn the_answer_depends_on_the_date_asked_about() {
    // By the end of November, Amina's external DG certificate has expired, with no grace.
    let answer = check(json!({
        "staff_number": 48213, "department": "Cargo", "role": "Handler", "on": "2026-11-30"
    }))
    .await;
    assert_eq!(
        answer.json()["report"]["lines"][1]["state"],
        json!({ "state": "expired", "valid_until": "2026-11-01" })
    );
}

#[tokio::test]
async fn an_unknown_employee_is_404() {
    let answer = check(json!({
        "staff_number": 99999, "department": "Cargo", "role": "Handler", "on": "2026-10-19"
    }))
    .await;
    assert_eq!(answer.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_unknown_role_is_422() {
    let answer = check(json!({
        "staff_number": 10007, "department": "Cargo", "role": "Pilot", "on": "2026-10-19"
    }))
    .await;
    assert_eq!(answer.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        answer.json()["message"],
        "no requirements defined for Pilot in Cargo"
    );
}
