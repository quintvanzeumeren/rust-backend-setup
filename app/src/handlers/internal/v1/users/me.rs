use axum::Json;
use serde::Serialize;
use uuid::Uuid;
use crate::extractors::admin::admin::Admin;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationResult;

#[derive(Serialize)]
pub struct MeResponse {
    pub user_id: Uuid,
    pub is_admin: bool,
}

#[tracing::instrument(
    name = "Get info about authenticated user",
    skip_all
)]
pub async fn me(
    authenticated_user: AuthenticatedUser,
    admin: Option<Admin>
) -> AuthenticationResult<Json<MeResponse>> {
    if let Some(admin) = admin {
        return Ok(Json(MeResponse {
            user_id: admin.authenticated_user.user_id.0,
            is_admin: true
        }))
    }

    return Ok(Json(MeResponse {
        user_id: authenticated_user.user_id.0,
        is_admin: false
    }))
}