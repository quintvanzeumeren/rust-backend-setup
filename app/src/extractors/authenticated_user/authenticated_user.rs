use std::sync::Arc;

use axum::{async_trait, RequestPartsExt};
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use secrecy::Secret;
use uuid::Uuid;

use domain::sessions::tokens::AccessToken;
use domain::sessions::user_session_token::UserSessionToken;
use domain::user::user_id::UserId;
use security::encryption::decryptor::Decryptor;
use security::token::token::Token;

use crate::app_state::AppState;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;
use crate::telemetry::TelemetryRecord;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub state: Arc<AppState>,
    pub user_id: UserId,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthenticationError;
    
    // todo: figure out trace id can be matched with request.
    // Tracing does not seem to work properly, currently the trace id of an extractor does not match.
    // I assume this is duo to extractor being executed in a separate task prior to calling the handler.
    // Because of which there is not yet a context from the function
    
    #[tracing::instrument(
        name = "Received extract authenticated user request",
        skip(parts, state),
        fields(
            user_id = tracing::field::Empty,
            session_id = tracing::field::Empty,
            refresh_token_id = tracing::field::Empty,
        ),
    )]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| AuthenticationError::AccessTokenHeadersInvalid)?;

        let app_state: Arc<AppState> = Arc::from_ref(state);
        let cipher = app_state.new_token_encryptor();
        let access_token = Secret::new(bearer.token().to_string());
        let access_token: UserSessionToken<AccessToken> = cipher.decrypt(&access_token)?;

        access_token.get_custom_claims().user_id.record_in_telemetry("user_id");
        access_token.get_custom_claims().session_id.record_in_telemetry("session_id");
        access_token.get_custom_claims().refresh_token_id.record_in_telemetry("refresh_token_id");

        let token_is_invalid = access_token.expired() || !access_token.active();
        if token_is_invalid {
            return Err(AuthenticationError::TokenInvalid)
        }

        Ok(AuthenticatedUser {
            state: app_state,
            user_id: access_token.get_custom_claims().user_id.into(),
            session_id: access_token.get_custom_claims().session_id,
            refresh_token_id: access_token.get_custom_claims().refresh_token_id
        })
    }
}