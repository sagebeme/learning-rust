//! Task 4: is this request allowed?
//!
//! `is_authorized` is true only when the `Authorization` header is exactly `Bearer <token>`,
//! with the right token. A missing header, another scheme, an empty token on the server, or a
//! header that isn't valid text are all `false`.
//!
//! Compare the tokens with `constant_time_eq`: look at every byte even after finding a
//! difference, so how long the check takes doesn't reveal how much of a guess was right.

// Delete the next line when you start: it only hides "unused" warnings in the empty stub.
#![allow(unused)]

use axum::http::HeaderMap;
use axum::http::header::AUTHORIZATION;

pub fn is_authorized(headers: &HeaderMap, token: &str) -> bool {
    todo!()
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    todo!()
}
