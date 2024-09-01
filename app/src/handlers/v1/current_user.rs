use axum::Json;
use serde::Serialize;
use uuid::Uuid;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;

#[tracing::instrument(
    name = "Checking health of server via protected health check",
    skip(current_user)
)]
pub async fn current_user(current_user: AuthenticatedUser) -> Json<CurrentUser> {
    return Json(current_user.into())
}

#[derive(Serialize, Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid
}

impl From<AuthenticatedUser> for CurrentUser {
    fn from(value: AuthenticatedUser) -> Self {
        Self {
            user_id: value.user_id.0,
            session_id: value.session_id,
            refresh_token_id: value.refresh_token_id,
        }
    }
}

