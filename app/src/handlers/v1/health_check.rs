use axum::http::StatusCode;

#[tracing::instrument(
    name = "Checking health of server"
)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
