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

use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use lib_auth::security::token::token::{Decryptor, Token};
use lib_domain::sessions::tokens::AccessToken;

use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;
use crate::routes::AppState;

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
        let access_token = Secret::new(bearer.token().to_string());
        let access_token: LocalPasetoV4Token<AccessToken> = LocalPasetoV4Token::decrypt(
            &access_token,
            &app_state.encryption_key
        )?;

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