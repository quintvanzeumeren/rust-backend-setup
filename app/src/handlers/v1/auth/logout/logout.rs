use axum::http::StatusCode;

use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::handlers::v1::auth::authentication_error::AuthenticationResult;

#[tracing::instrument(
    name = "Logging out user by invalidating user session",
    skip(authenticated_user)
)]
pub async fn logout(
    authenticated_user: AuthenticatedUser
) -> AuthenticationResult<StatusCode> {
    authenticated_user.logout().await?;
    Ok(StatusCode::OK)
}