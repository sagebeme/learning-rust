//! Task 4: is this request allowed?
//!
//! `is_authorized` is true only when the `Authorization` header is exactly `Bearer <token>`,
//! with the right token. A missing header, another scheme, an empty token on the server, or a
//! header that isn't valid text are all `false`.
//!
//! Compare the tokens with `constant_time_eq`: look at every byte even after finding a
//! difference, so how long the check takes doesn't reveal how much of a guess was right.

use axum::http::HeaderMap;
use axum::http::header::AUTHORIZATION;

pub fn is_authorized(headers: &HeaderMap, token: &str) -> bool {
    if token.is_empty() {
        return false;
    }
    let Some(sent) = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return false;
    };
    constant_time_eq(sent.as_bytes(), token.as_bytes())
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    // XOR every pair of bytes and OR the results together: no early exit, so the time taken
    // doesn't depend on where the first difference is.
    a.iter().zip(b).fold(0u8, |diff, (x, y)| diff | (x ^ y)) == 0
}
