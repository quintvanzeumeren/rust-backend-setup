use std::sync::Arc;

use anyhow::Context;
use axum::extract::{FromRef, FromRequest, FromRequestParts};
use axum::http::request::Parts;
use axum::{async_trait, RequestExt};
use uuid::Uuid;

use domain::user::user_id::UserId;

use crate::app_state::AppState;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;
use crate::policy::policy::Policy;

pub struct UserWith<T> {
    pub content: T,
    pub user_id: UserId,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid,
}

#[async_trait]
impl <S, P> FromRequestParts<S> for UserWith<P>
    where
        S: Send + Sync,
        Arc<AppState>: FromRef<S>,
        P: Policy,
{
    type Rejection = AuthenticationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let mut mutable_parts = parts.clone();
        let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;

        let policy = P::new(authenticated_user.state, authenticated_user.user_id)
            .await
            .context("Failed to initialise Policy")?;

        Ok(Self {
            content: policy,
            user_id: authenticated_user.user_id,
            session_id: authenticated_user.session_id,
            refresh_token_id: authenticated_user.refresh_token_id,
        })
    }
}