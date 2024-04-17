use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use lib_auth::security::token::token::{Decryptor, Encryptor, Token};
use lib_domain::sessions::tokens::RefreshToken;

use crate::handlers::internal::v1::auth::authentication_error::{AuthenticationError, AuthenticationResult};
use crate::queries::get_active_session_by_id::get_active_session_by_id;
use crate::queries::save_just_ended_user_session::save_just_ended_session;
use crate::queries::save_refreshed_user_session::save_refreshed_session;
use crate::routes::AppState;
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(Deserialize)]
pub struct RefreshRequest {
    refresh_token: Secret<String>
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
    refresh_request: Json<RefreshRequest>
) -> AuthenticationResult<(StatusCode, Json<RefreshResponse>)> {

    // TODO: put decrypt in tokio blocking
    let refresh_token = LocalPasetoV4Token::<RefreshToken>::decrypt(
        &refresh_request.refresh_token, &state.encryption_key
    )?;

    tracing::Span::current().record("user_id", &tracing::field::display(&refresh_token.get_custom_claims().user_id));
    tracing::Span::current().record("session_id", &tracing::field::display(&refresh_token.get_custom_claims().session_id));
    tracing::Span::current().record("refresh_token_id", &tracing::field::display(&refresh_token.get_id()));

    let active_session = get_active_session_by_id(&state.db, &refresh_token.get_custom_claims().session_id)
        .await
        .context("Failed to active session from Postgres")?
        .ok_or(AuthenticationError::SessionNotActive)?;

    tracing::Span::current().record("latest_session_refresh_token_id", &tracing::field::display(&active_session.state().latest_refresh_token.id));

    match active_session.refresh(refresh_token) {
        Ok(refresh_session) => {

            let mut transaction = state.db.begin()
                .await
                .context("Failed to start transaction to save refreshed session")?;

            save_refreshed_session(&mut transaction, &refresh_session)
                .await
                .context("Failed to save refresh session to Postgres")?;

            let (
                access_token,
                refresh_token
            ) = spawn_blocking_with_tracing(move || {
                let encryption_access_result = refresh_session.state()
                    .new_access_token()
                    .encrypt(&state.encryption_key);

                let encryption_refresh_result = refresh_session.state()
                    .new_refresh_token()
                    .encrypt(&state.encryption_key);

                (encryption_access_result, encryption_refresh_result)
            }).await.context("Failed to spawn blocking tokio task to encrypt refreshed session tokens")?;

            let access_token = access_token.context("Failed to encrypt access token")?;
            let refresh_token = refresh_token.context("Failed to encrypt refresh token")?;

            transaction.commit().await.context("Failed to commit refresh token to Postgres")?;

            Ok((StatusCode::CREATED, Json(
                RefreshResponse {
                    access_token: access_token.token.expose_secret().clone(),
                    access_token_expiration: access_token.expires_at,
                    refresh_token: refresh_token.token.expose_secret().clone(),
                    refresh_token_expiration: refresh_token.expires_at,
                }
            )))
        }
        Err(ended_session) => {
            let mut transaction = state.db.begin()
                .await
                .context("Failed to start transaction to save refreshed session")?;

            save_just_ended_session(&mut transaction, &ended_session)
                .await
                .context("Failed to save refresh session to Postgres")?;

            transaction.commit().await.context("Failed to commit just ended token to Postgres")?;

            Err(AuthenticationError::TokenInvalid)
        }
    }
}