use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Duration, Utc};
use domain::sessions::state::just_ended::JustEnded;
use domain::sessions::state::refreshed::Refreshed;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use domain::sessions::tokens::RefreshToken;
use domain::sessions::user_session::UserSession;
use domain::sessions::user_session_token::UserSessionToken;
use infrastructure::paseto::paseto_token_encryptor::LocalPasetoV4DecryptionError;
use security::encryption::decryptor::Decryptor;
use security::encryption::encryptor::Encryptor;
use security::token::token::Token;

use crate::app_state::AppState;
use crate::handlers::v1::auth::authentication_error::{
    AuthenticationError, AuthenticationResult,
};
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(Deserialize)]
pub struct RefreshRequest {
    refresh_token: Secret<String>,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    access_token: String,
    access_token_expiration: DateTime<Utc>,
    refresh_token: String,
    refresh_token_expiration: DateTime<Utc>,
}

#[tracing::instrument(
    name = "Refreshing access and refresh tokens for user",
    skip(state, refresh_request),
    fields (
        user_id = tracing::field::Empty,
        session_id = tracing::field::Empty,
        refresh_token_id = tracing::field::Empty,
        latest_session_refresh_token_id=tracing::field::Empty
    )
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    refresh_request: Json<RefreshRequest>,
) -> AuthenticationResult<(StatusCode, Json<RefreshResponse>)> {
    let token_encryptor = state.new_token_encryptor();
    let refresh_token = spawn_blocking_with_tracing(move || {
        let refresh_token: Result<UserSessionToken<RefreshToken>, LocalPasetoV4DecryptionError> =
            token_encryptor.decrypt(&refresh_request.refresh_token);

        refresh_token
    })
        .await
        .context("Failed to spawn blocking tokio task to encrypt refreshed session tokens")??;

    tracing::Span::current().record(
        "user_id", &tracing::field::display(&refresh_token.get_custom_claims().user_id),
    );
    tracing::Span::current().record(
        "session_id", &tracing::field::display(&refresh_token.get_custom_claims().session_id),
    );
    tracing::Span::current().record(
        "refresh_token_id", &tracing::field::display(&refresh_token.get_id()),
    );

    let active_session = state.db
        .get_active_session_by_id(&refresh_token.get_custom_claims().session_id)
        .await
        .context("Failed to active session from Postgres")?
        .ok_or(AuthenticationError::SessionNotActive)?;

    tracing::Span::current().record(
        "latest_session_refresh_token_id",
        &tracing::field::display(&active_session.state().latest_refresh_token.id),
    );

    return match active_session.refresh(refresh_token) {
        Ok(refreshed_session) => {
            save_refreshed_session_and_generate_response(state, refreshed_session).await
        }
        Err(ended_session) => save_ended_session_and_generate_response(state, ended_session).await,
    };
}

async fn save_refreshed_session_and_generate_response(
    state: Arc<AppState>,
    refreshed_session: UserSession<Refreshed>,
) -> AuthenticationResult<(StatusCode, Json<RefreshResponse>)> {
    let mut transaction = state.db.new_transaction().await
        .context("Failed to start transaction to save refreshed session")?;

    transaction.save_refreshed_session(&refreshed_session).await
        .context("Failed to save refresh session to Postgres")?;

    let token_encryptor = state.new_token_encryptor();
    let (access_token, refresh_token) = spawn_blocking_with_tracing(move || {
        let encryption_access_result =
            token_encryptor.encrypt(refreshed_session.state().new_access_token());

        let encryption_refresh_result =
            token_encryptor.encrypt(refreshed_session.state().new_refresh_token());

        (encryption_access_result, encryption_refresh_result)
    })
    .await
    .context("Failed to spawn blocking tokio task to encrypt refreshed session tokens")?;

    let access_token = access_token.context("Failed to encrypt access token")?;
    let refresh_token = refresh_token.context("Failed to encrypt refresh token")?;

    transaction.commit().await
        .context("Failed to commit refresh token to Postgres")?;

    Ok((
        StatusCode::CREATED,
        Json(RefreshResponse {
            // we are removing 30 seconds from the actual expiration time, to increase the
            // likelihood of the token refreshing the tokens on time
            access_token: access_token.token.expose_secret().clone(),
            access_token_expiration: access_token.expires_at - Duration::seconds(30),
            refresh_token: refresh_token.token.expose_secret().clone(),
            refresh_token_expiration: refresh_token.expires_at - Duration::seconds(30),
        }),
    ))
}

async fn save_ended_session_and_generate_response(
    state: Arc<AppState>,
    ended_session: UserSession<JustEnded>,
) -> AuthenticationResult<(StatusCode, Json<RefreshResponse>)> {
    let mut transaction = state.db.new_transaction().await
        .context("Failed to start transaction to save refreshed session")?;

    transaction.save_just_ended_session(&ended_session).await
        .context("Failed to save refresh session to Postgres")?;

    transaction.commit().await
        .context("Failed to commit just ended token to Postgres")?;

    Err(AuthenticationError::TokenInvalid)
}
