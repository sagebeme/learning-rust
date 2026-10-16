mod common;

use async_backend::app::app;
use async_backend::t04_auth::{constant_time_eq, is_authorized};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use common::{TOKEN, call, state};

fn headers(value: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(value).expect("a valid header"),
    );
    headers
}

#[test]
fn accepts_the_right_bearer_token() {
    assert!(is_authorized(&headers(&format!("Bearer {TOKEN}")), TOKEN));
}

#[test]
fn rejects_everything_else() {
    assert!(!is_authorized(&HeaderMap::new(), TOKEN));
    assert!(!is_authorized(&headers("Bearer wrong"), TOKEN));
    assert!(!is_authorized(&headers(TOKEN), TOKEN));
    assert!(!is_authorized(&headers(&format!("Basic {TOKEN}")), TOKEN));
    assert!(!is_authorized(&headers(&format!("Bearer {TOKEN}x")), TOKEN));
    assert!(!is_authorized(&headers("Bearer "), ""));
}

#[test]
fn rejects_a_header_that_is_not_text() {
    let mut map = HeaderMap::new();
    map.insert(
        "authorization",
        HeaderValue::from_bytes(b"Bearer \xff").unwrap(),
    );
    assert!(!is_authorized(&map, TOKEN));
}

#[test]
fn compares_byte_for_byte() {
    assert!(constant_time_eq(b"abc", b"abc"));
    assert!(!constant_time_eq(b"abc", b"abd"));
    assert!(!constant_time_eq(b"abc", b"ab"));
    assert!(constant_time_eq(b"", b""));
}

#[tokio::test]
async fn the_server_requires_the_token() {
    let without = call(app(state()), "GET", "/employees/48213", None, None).await;
    assert_eq!(without.status, StatusCode::UNAUTHORIZED);
    assert_eq!(without.json()["error"], "unauthorized");

    let with = call(app(state()), "GET", "/employees/48213", None, Some(TOKEN)).await;
    assert_eq!(with.status, StatusCode::OK);
}

#[tokio::test]
async fn health_stays_open_for_monitoring() {
    let answer = call(app(state()), "GET", "/health", None, None).await;
    assert_eq!(answer.status, StatusCode::OK);
}
