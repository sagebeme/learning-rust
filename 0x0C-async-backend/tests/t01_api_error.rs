use async_backend::t01_api_error::ApiError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use serde_json::{Value, json};

async fn render(error: ApiError) -> (StatusCode, Value) {
    let response = error.into_response();
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("a body")
        .to_bytes();
    (status, serde_json::from_slice(&bytes).expect("a JSON body"))
}

#[tokio::test]
async fn each_error_has_its_status_and_code() {
    let cases = [
        (
            ApiError::NotFound("employee 9".into()),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            ApiError::Conflict("taken".into()),
            StatusCode::CONFLICT,
            "conflict",
        ),
        (
            ApiError::Invalid("bad".into()),
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid",
        ),
        (
            ApiError::Unauthorized,
            StatusCode::UNAUTHORIZED,
            "unauthorized",
        ),
        (
            ApiError::Internal("x".into()),
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal",
        ),
    ];
    for (error, status, code) in cases {
        assert_eq!(error.status(), status);
        assert_eq!(error.code(), code);
        let (rendered_status, body) = render(error).await;
        assert_eq!(rendered_status, status);
        assert_eq!(body["error"], code);
    }
}

#[tokio::test]
async fn messages_are_written_for_people() {
    assert_eq!(
        render(ApiError::NotFound("employee 48213".into())).await.1,
        json!({ "error": "not_found", "message": "employee 48213 was not found" })
    );
    assert_eq!(
        render(ApiError::Invalid("name can't be blank".into()))
            .await
            .1["message"],
        "name can't be blank"
    );
    assert_eq!(
        render(ApiError::Unauthorized).await.1["message"],
        "a valid bearer token is required"
    );
}

#[tokio::test]
async fn internal_details_never_reach_the_client() {
    let (_, body) = render(ApiError::Internal(
        "relation \"holdings\" does not exist".into(),
    ))
    .await;
    assert_eq!(
        body,
        json!({ "error": "internal", "message": "something went wrong" })
    );
}
