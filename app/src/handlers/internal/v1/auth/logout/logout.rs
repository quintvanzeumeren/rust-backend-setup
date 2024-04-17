use std::sync::Arc;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;

use crate::extractors::authenticated_user::AuthenticatedUser;
use crate::handlers::internal::v1::auth::authentication_error::{AuthenticationError, AuthenticationResult};
use crate::queries::get_active_session_by_id::get_active_session_by_id;
use crate::queries::save_just_ended_user_session::save_just_ended_session;
use crate::routes::AppState;

#[tracing::instrument(
    name = "Logging out user by invalidating user session",
    skip(state, authenticated_user)
)]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    authenticated_user: AuthenticatedUser
) -> AuthenticationResult<StatusCode> {

    let session = get_active_session_by_id(&state.db, &authenticated_user.session_id)
        .await
        .context("Failed to query database to get active session")?
        .ok_or(AuthenticationError::SessionNotActive)?
        .end_by_logout();

    let mut transaction = state.db.begin()
        .await
        .expect("Failed to begin a transaction to store updated user session");

    save_just_ended_session(&mut transaction, &session)
        .await
        .context("Failed to save updated user session to database")?;

    transaction.commit().await.context("Failed to commit transaction containing updated database")?;

    Ok(StatusCode::OK)
}