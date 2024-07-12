use axum::body::{Body, Bytes};
use axum::extract::Request;
use axum::http::{Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use tracing::info;

#[tracing::instrument(
    skip_all
)]
pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // middleware to capture trace before user extractors.
    // Extractors are captured within a different trace prior receiving the handler,
    // thereby not capturing the full trace that reaches the server.
    // info!("Before request: {}", req.uri().path().to_string());

    let res = next.run(req).await;

    // info!("after request");

    Ok(res)
}