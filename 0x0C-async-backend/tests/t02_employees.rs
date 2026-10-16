mod common;

use async_backend::app::routes;
use axum::http::StatusCode;
use common::{call, state};
use serde_json::json;

// /health is given (in app.rs), so this test passes from the start.
#[tokio::test]
async fn health_is_ok() {
    let answer = call(routes(state()), "GET", "/health", None, None).await;
    assert_eq!(answer.status, StatusCode::OK);
    assert_eq!(answer.text, "ok");
}

#[tokio::test]
async fn gets_an_employee() {
    let answer = call(routes(state()), "GET", "/employees/48213", None, None).await;
    assert_eq!(answer.status, StatusCode::OK);
    assert_eq!(
        answer.json(),
        json!({
            "staff_number": 48213,
            "name": "Amina Wanjiru",
            "station": "NBO",
            "department": "Ramp",
            "codes": ["AIRSIDE", "FIRE"]
        })
    );
}

#[tokio::test]
async fn an_unknown_employee_is_404() {
    let answer = call(routes(state()), "GET", "/employees/99999", None, None).await;
    assert_eq!(answer.status, StatusCode::NOT_FOUND);
    assert_eq!(answer.json()["message"], "employee 99999 was not found");
}

#[tokio::test]
async fn creates_an_employee() {
    let state = state();
    let body = json!({ "staff_number": 55555, "name": " Njeri Kamau ", "station": "EDL", "department": "Cargo" });
    let answer = call(
        routes(state.clone()),
        "POST",
        "/employees",
        Some(body),
        None,
    )
    .await;
    assert_eq!(answer.status, StatusCode::CREATED);
    assert_eq!(answer.json()["name"], "Njeri Kamau");

    let fetched = call(routes(state), "GET", "/employees/55555", None, None).await;
    assert_eq!(fetched.status, StatusCode::OK);
    assert_eq!(fetched.json()["codes"], json!([]));
}

#[tokio::test]
async fn a_taken_staff_number_is_a_conflict() {
    let body =
        json!({ "staff_number": 48213, "name": "Someone", "station": "NBO", "department": "Ramp" });
    let answer = call(routes(state()), "POST", "/employees", Some(body), None).await;
    assert_eq!(answer.status, StatusCode::CONFLICT);
    assert_eq!(
        answer.json()["message"],
        "staff number 48213 already exists"
    );
}

#[tokio::test]
async fn bad_input_is_422_with_the_reason() {
    let cases = [
        (
            json!({ "staff_number": 0, "name": "A", "station": "NBO", "department": "Ramp" }),
            "staff number must be above 0",
        ),
        (
            json!({ "staff_number": 1, "name": "  ", "station": "NBO", "department": "Ramp" }),
            "name can't be blank",
        ),
        (
            json!({ "staff_number": 1, "name": "A", "station": "NBO", "department": "" }),
            "department can't be blank",
        ),
        (
            json!({ "staff_number": 1, "name": "A", "station": "LHR", "department": "Ramp" }),
            "unknown station LHR",
        ),
    ];
    for (body, message) in cases {
        let answer = call(routes(state()), "POST", "/employees", Some(body), None).await;
        assert_eq!(answer.status, StatusCode::UNPROCESSABLE_ENTITY, "{message}");
        assert_eq!(answer.json()["message"], message);
    }
}
