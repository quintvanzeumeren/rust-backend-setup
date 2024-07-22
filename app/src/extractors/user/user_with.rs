use std::sync::Arc;

use anyhow::Context;
use axum::{async_trait, Json, RequestExt};
use axum::extract::{FromRef, FromRequest, FromRequestParts, Path, Request};
use axum::http::request::Parts;
use serde::de::DeserializeOwned;

use domain::permission::permission::Permission;
use crate::app_state::AppState;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::extractors::user::permission_extractor::permission_of::PermissionOf;
use crate::extractors::user::user_extractor::{UserContent, UserExtractor};
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;

struct UserWith<Extractor: UserContent> {
    pub content: Extractor::Content,
}

#[async_trait]
impl<P, S, D> FromRequest<S> for UserWith<PermissionOf<P, D>>
where
    S: Send + Sync,
    P: Permission,
    D: DeserializeOwned + Into<P::Context> + Send + Sync,
    Arc<AppState>: FromRef<S>,
    PermissionOf<P, D>: UserExtractor<Content = P>
{
    type Rejection = AuthenticationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let mut mutable_parts = parts.clone();
        let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;

        let req = Request::from_parts(parts, body);
        let body: Json<D> = Json::from_request(req, state)
            .await
            .map_err(|e| AuthenticationError::JsonRejection(e))?;
        
        let permission_extractor: PermissionOf<P, D> = PermissionOf::create(authenticated_user.state.db.clone());
        let permission = permission_extractor.extract(authenticated_user.user_id)
            .await
            .context("Failed to extract permission for user")?;

        if !permission.is_authorized(body.0.into()) {
            return Ok(Self {
                content: permission
            })
        }

        Err(AuthenticationError::UnAuthorized)
    }
}

#[async_trait]
impl<P, S, D> FromRequestParts<S> for UserWith<PermissionOf<P, D>>
where
    S: Send + Sync,
    P: Permission,
    D: DeserializeOwned + Into<P::Context> + Send + Sync,
    Arc<AppState>: FromRef<S>,
    PermissionOf<P, D>: UserExtractor<Content = P>
{
    type Rejection = AuthenticationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let mut mutable_parts = parts.clone();
        let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;
        
        let params: Path<D> = Path::from_request_parts(parts, state).await.expect("todo handle error");
        let permission_extractor: PermissionOf<P, D> = PermissionOf::create(authenticated_user.state.db.clone());
        let permission = permission_extractor.extract(authenticated_user.user_id)
            .await
            .context("Failed to extract permission for user")?;

        if !permission.is_authorized(params.0.into()) {
            return Ok(Self {
                content: permission
            })
        }

        Err(AuthenticationError::UnAuthorized)
    }
}

