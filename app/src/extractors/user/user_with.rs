use std::sync::Arc;

use anyhow::Context;
use axum::{async_trait, Json, RequestExt};
use axum::extract::{FromRef, FromRequest, FromRequestParts, Request};
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
    PermissionOf<P, D>: UserExtractor<P>
{
    type Rejection = AuthenticationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let mut mutable_parts = parts.clone();
        let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;

        let req = Request::from_parts(parts, body);
        let body: Json<D> = Json::from_request(req, state).await.expect("todo proper error handling");

        let permission_extractor: PermissionOf<P, D> = PermissionOf::create(authenticated_user.state.db.clone());
        let permission = permission_extractor.extract(authenticated_user.user_id)
            .await
            .context("Failed to extract permission for user")?;
        
        let has_permission = permission.is_granted_for(body.0.into());
        if !has_permission {
        //     todo!("Return proper error response")
        }

        todo!("Return proper object back to handler")
    }
}