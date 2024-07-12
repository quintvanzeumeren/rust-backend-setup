use std::sync::Arc;
use anyhow::Context;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use crate::app_state::AppState;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;

pub struct Admin {
    pub authenticated_user: AuthenticatedUser
}

impl Admin {
    pub(super) fn state(&self) -> &Arc<AppState> {
        &self.authenticated_user.state
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Admin where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthenticationError;

    #[tracing::instrument(
        name="Received extract admin request",
        skip_all,
    )]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let authenticated_user = AuthenticatedUser::from_request_parts(parts, state).await?;

        // todo add logic to verify admin role
        
        // let is_admin = authenticated_user.state.db.is_admin(&authenticated_user.user_id)
        //     .await
        //     .context("An error occurred while checking authenticated user is an admin")?;
        // if is_admin {
        //     return Ok(Self {
        //         authenticated_user
        //     })
        // }

        return Err(AuthenticationError::AuthenticatedUserIsNotOfTypeAdmin)
    }
}

