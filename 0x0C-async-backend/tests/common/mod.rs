//! Shared helpers: send a request straight to the router (no network) and read the answer.
#![allow(dead_code)]

use async_backend::model::AppState;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

pub const TOKEN: &str = "test-token-not-a-real-secret";

pub fn state() -> AppState {
    AppState::with_sample_data(TOKEN)
}

pub struct Answer {
    pub status: StatusCode,
    pub text: String,
}

impl Answer {
    pub fn json(&self) -> Value {
        serde_json::from_str(&self.text)
            .unwrap_or_else(|_| panic!("expected a JSON body, got {:?}", self.text))
    }
}

pub async fn call(
    router: Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
    bearer: Option<&str>,
) -> Answer {
    let mut request = Request::builder().method(method).uri(uri);
    if let Some(token) = bearer {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let request = match body {
        Some(json) => request
            .header("content-type", "application/json")
            .body(Body::from(json.to_string())),
        None => request.body(Body::empty()),
    }
    .expect("a valid request");

    let response = router.oneshot(request).await.expect("the router answers");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("a readable body")
        .to_bytes();
    Answer {
        status,
        text: String::from_utf8_lossy(&bytes).into_owned(),
    }
}
