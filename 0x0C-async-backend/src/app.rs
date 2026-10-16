//! Given: which URL goes to which handler. Read this to see how the pieces fit together.

use axum::Router;
use axum::extract::{Request, State};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use crate::model::AppState;
use crate::t01_api_error::ApiError;
use crate::{t02_employees, t03_transfers, t04_auth, t100_list};

async fn health() -> &'static str {
    "ok"
}

/// Every route, with no authentication. Tests for tasks 1–3 and 100 use this.
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route(
            "/employees",
            get(t100_list::list_employees).post(t02_employees::create_employee),
        )
        .route(
            "/employees/{staff_number}",
            get(t02_employees::get_employee),
        )
        .route("/transfers/check", post(t03_transfers::check_transfer))
        .with_state(state)
}

/// Checks the bearer token on every request except `/health`, using task 4.
async fn require_token(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if request.uri().path() == "/health" || t04_auth::is_authorized(request.headers(), &state.token)
    {
        next.run(request).await
    } else {
        ApiError::Unauthorized.into_response()
    }
}

/// The real server: every route, behind the token check.
pub fn app(state: AppState) -> Router {
    routes(state.clone()).layer(middleware::from_fn_with_state(state, require_token))
}
