use std::sync::Arc;

use axum::{async_trait, RequestPartsExt};
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use secrecy::Secret;
use serde::Serialize;
use uuid::Uuid;
use security::encryption::decryptor::Decryptor;
use security::token::token::Token;
use domain::sessions::user_session_token::UserSessionToken;
use domain::sessions::tokens::AccessToken;
use crate::app_state::AppState;

use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;


#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid
}

// TODO: write additional integration tests that submit the following:
// - inactive tokens,
// - expired tokens,
// - invalid encrypted token
// - incorrect headers

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthenticationError;
    #[tracing::instrument(
        name = "Verifying access token",
        skip(parts, state)
        fields(
            user_id = tracing::field::Empty,
            session_id = tracing::field::Empty,
            refresh_token_id = tracing::field::Empty,
        )
    )]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| AuthenticationError::AccessTokenHeadersInvalid)?;

        let app_state: Arc<AppState> = Arc::from_ref(state);
        let cipher = app_state.new_token_encryptor();
        let access_token = Secret::new(bearer.token().to_string());
        let access_token: UserSessionToken<AccessToken> = cipher.decrypt(&access_token)?;

        tracing::Span::current().record("user_id", &tracing::field::display(&access_token.get_custom_claims().user_id));
        tracing::Span::current().record("session_id", &tracing::field::display(&access_token.get_custom_claims().session_id));
        tracing::Span::current().record("refresh_token_id", &tracing::field::display(&access_token.get_custom_claims().refresh_token_id));

        let token_is_invalid = access_token.expired() || !access_token.active();
        if token_is_invalid {
            return Err(AuthenticationError::TokenInvalid)
        }

        Ok(AuthenticatedUser {
            user_id: access_token.get_custom_claims().user_id,
            session_id: access_token.get_custom_claims().session_id,
            refresh_token_id: access_token.get_custom_claims().refresh_token_id
        })
    }
}