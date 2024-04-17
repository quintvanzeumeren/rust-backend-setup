use axum::Json;

use crate::extractors::authenticated_user::AuthenticatedUser;

#[tracing::instrument(
    name = "Checking health of server via protected health check",
    skip(current_user)
)]
pub async fn current_user(current_user: AuthenticatedUser) -> Json<AuthenticatedUser> {
    return Json(current_user)
}