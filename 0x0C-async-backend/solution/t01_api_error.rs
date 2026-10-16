//! Task 1: every error becomes the right HTTP response.
//!
//! | Variant | Status | `error` |
//! | --- | --- | --- |
//! | `NotFound(what)` | 404 | `"not_found"` |
//! | `Conflict(why)` | 409 | `"conflict"` |
//! | `Invalid(why)` | 422 | `"invalid"` |
//! | `Unauthorized` | 401 | `"unauthorized"` |
//! | `Internal(detail)` | 500 | `"internal"` |
//!
//! The body is JSON: `{"error": <code>, "message": <public_message()>}`. The public message is
//! the variant's text, except: `Unauthorized` → `"a valid bearer token is required"`, and
//! `Internal` → `"something went wrong"`. Never send an `Internal` detail to the client: print it
//! with `eprintln!` instead (a real server logs it).

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    NotFound(String),
    Conflict(String),
    Invalid(String),
    Unauthorized,
    Internal(String),
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Invalid(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            ApiError::NotFound(_) => "not_found",
            ApiError::Conflict(_) => "conflict",
            ApiError::Invalid(_) => "invalid",
            ApiError::Unauthorized => "unauthorized",
            ApiError::Internal(_) => "internal",
        }
    }

    pub fn public_message(&self) -> String {
        match self {
            ApiError::NotFound(what) => format!("{what} was not found"),
            ApiError::Conflict(why) | ApiError::Invalid(why) => why.clone(),
            ApiError::Unauthorized => "a valid bearer token is required".to_string(),
            ApiError::Internal(_) => "something went wrong".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if let ApiError::Internal(detail) = &self {
            eprintln!("internal error: {detail}");
        }
        let body = json!({ "error": self.code(), "message": self.public_message() });
        (self.status(), Json(body)).into_response()
    }
}
