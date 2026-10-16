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

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

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
        todo!()
    }

    pub fn code(&self) -> &'static str {
        todo!()
    }

    pub fn public_message(&self) -> String {
        todo!()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        todo!()
    }
}
