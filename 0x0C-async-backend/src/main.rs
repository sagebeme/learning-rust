//! Given: run the mini-TWA server for real, with the course's sample data.
//!
//! ```bash
//! cargo run -p async-backend
//! ```
//!
//! Then, in another terminal:
//!
//! ```bash
//! curl http://127.0.0.1:3000/health
//! curl -H "Authorization: Bearer dev-token" http://127.0.0.1:3000/employees/48213
//! curl -X POST http://127.0.0.1:3000/transfers/check \
//!      -H "Authorization: Bearer dev-token" -H "Content-Type: application/json" \
//!      -d '{"staff_number": 48213, "department": "Cargo", "role": "Handler", "on": "2026-10-19"}'
//! ```
//!
//! Every endpoint calls your task code, so each one works once its task is done.

use async_backend::app::app;
use async_backend::model::AppState;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // A real server reads its token from a secret store, never from code. This default is only for
    // trying the server on your own machine.
    let token = std::env::var("TWA_TOKEN").unwrap_or_else(|_| "dev-token".to_string());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("mini-TWA server listening on http://127.0.0.1:3000");
    axum::serve(listener, app(AppState::with_sample_data(&token))).await
}
