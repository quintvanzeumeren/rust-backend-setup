use std::sync::Arc;

use anyhow::Context;
use axum::{async_trait, Json, RequestExt};
use axum::extract::{FromRef, FromRequest, FromRequestParts, Request};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use domain::permission::permission_authorizer::PermissionAuthorizer;
use domain::user::user_id::UserId;

use crate::app_state::AppState;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::extractors::user::permission_extractor::permission_of::PermissionOf;
use crate::extractors::user::user_extractor::{UserContent, UserExtractor};
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;

pub struct UserWith<Extractor: UserContent> {
    pub content: Extractor::Content,
    pub request_content: Extractor::RequestContent,
    
    pub state: Arc<AppState>,
    pub user_id: UserId,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid
}

#[async_trait]
impl<P, S, Body> FromRequest<S> for UserWith<PermissionOf<P, Body>>
where
    S: Send + Sync,
    P: PermissionAuthorizer,
    Body: DeserializeOwned + Into<P::ResourceInQuestion> + Send + Sync + Clone,
    Arc<AppState>: FromRef<S>,
    PermissionOf<P, Body>: UserExtractor<Content = P, RequestContent = Body>
{
    type Rejection = AuthenticationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let mut mutable_parts = parts.clone();
        let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;

        let req = Request::from_parts(parts, body);
        let body: Json<Body> = Json::from_request(req, state)
            .await
            .map_err(|e| AuthenticationError::JsonRejection(e))?;

        let permission_extractor: PermissionOf<P, Body> = PermissionOf::create(authenticated_user.state.db.clone());
        let permission = permission_extractor.extract(&authenticated_user.user_id)
            .await
            .context("Failed to extract permission for user")?;
        
        if !permission.is_authorized_for(body.0.clone().into()) {
            return Ok(Self {
                content: permission,
                request_content: body.0,
                state: authenticated_user.state,
                user_id: authenticated_user.user_id,
                session_id: authenticated_user.session_id,
                refresh_token_id: authenticated_user.refresh_token_id,
            })
        }

        Err(AuthenticationError::UnAuthorized)
    }
}

// #[async_trait]
// impl<P, S, Params> FromRequestParts<S> for UserWith<PermissionOf<P, Params>>
// where
//     S: Send + Sync,
//     P: PermissionAuthorizer,
//     Params: DeserializeOwned + Into<P::ResourceInQuestion> + Send + Sync + Clone,
//     Arc<AppState>: FromRef<S>,
//     PermissionOf<P, Params>: UserExtractor<Content = P, RequestContent = Params>
// {
//     type Rejection = AuthenticationError;
// 
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let mut mutable_parts = parts.clone();
//         let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;
// 
//         let params: Path<Params> = Path::from_request_parts(parts, state).await.expect("todo handle error");
//         let permission_extractor: PermissionOf<P, Params> = PermissionOf::create(authenticated_user.state.db.clone());
//         let permission = permission_extractor.extract(&authenticated_user.user_id)
//             .await
//             .context("Failed to extract permission for user")?;
//         
//         if !permission.is_authorized_for(params.0.clone().into()) {
//             return Ok(Self {
//                 content: permission,
//                 request_content: params.0,
//                 state: authenticated_user.state,
//                 user_id: authenticated_user.user_id,
//                 session_id: authenticated_user.session_id,
//                 refresh_token_id: authenticated_user.refresh_token_id,
//             })
//         }
// 
//         Err(AuthenticationError::UnAuthorized)
//     }
// }

