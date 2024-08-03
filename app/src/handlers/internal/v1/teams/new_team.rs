use axum::http::StatusCode;


#[tracing::instrument(
    name = "Adding a new team"
)]
pub async fn new_team() -> StatusCode {
    todo!("Implement")
}