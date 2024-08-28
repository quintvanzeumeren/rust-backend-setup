use std::sync::Arc;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Duration, Utc};
use password_hash::SaltString;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;

use domain::sessions::state::newly_created::NewlyCreated;
use domain::sessions::user_session::UserSession;
use domain::user::password::{MatchError, MatchResult, Password};
use security::encryption::encryptor::Encryptor;

use crate::app_state::AppState;
use crate::handlers::v1::auth::authentication_error::{
    AuthenticationError, AuthenticationResult,
};
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(Deserialize)]
pub struct LoginRequestBody {
    username: String,
    password: Secret<String>,
}

#[derive(Serialize)]
pub enum LoginResponse {
    UserLoggedInSuccessfully {
        access_token: String,
        access_token_expiration: DateTime<Utc>,
        refresh_token: String,
        refresh_token_expiration: DateTime<Utc>,
    },
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        match self {
            LoginResponse::UserLoggedInSuccessfully { .. } => {
                (StatusCode::OK, Json(self)).into_response()
            }
        }
    }
}

#[tracing::instrument(
    name = "Received user login request",
    skip(state, credentials),
    fields(
        user_id = tracing::field::Empty
    ),
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    credentials: Json<LoginRequestBody>,
) -> AuthenticationResult<LoginResponse> {
    // We Always need to verify the submitted password with a password hash,
    // even if the username is not found. This to prevent timing attacks and
    // user enumeration vulnerabilities.
    // See: https://en.wikipedia.org/wiki/Timing_attack
    // and https://owasp.org/www-project-web-security-testing-guide/latest/4-Web_Application_Security_Testing/03-Identity_Management_Testing/04-Testing_for_Account_Enumeration_and_Guessable_User_Account
    let mut user_id = None;
    let mut expected_user_password =
        get_dummy_hash().context("Failed to create default password")?;

    // todo move into queries crate
    let optional_user_credentials = state.db.get_user_credentials(&credentials.username)
        .await
        .context("Failed to get user credentials from Postgres")?;
    
    if let Some(user_credentials) = optional_user_credentials {
        user_id = Some(user_credentials.user_id);
        expected_user_password = Password::try_from(user_credentials.password_hash.expose_secret().clone())
            .context("Failed to parse password hash")?;
    }

    let match_result = verify_if_password_matches(expected_user_password, &credentials.password)
        .await
        .context("Failed to spawn blocking tokio task to verify password")?
        .context("Failed to submitted password with expected password")?;

    let user_id = user_id.ok_or(AuthenticationError::CredentialsInvalid)?;
    let mut transaction = state
        .db
        .new_transaction()
        .await
        .context("Failed to start a Postgres transaction")?;

    match match_result {
        MatchResult::DoesNotMatch => return Err(AuthenticationError::CredentialsInvalid),
        MatchResult::Matches => {}
        MatchResult::MatchesButSchemeOutdated => {
            // TODO: move this code in its own function
            // TODO: shouldn't fail entire login operation if we cannot update the password, better to just log it
            let password = credentials.password.clone();
            let hash_result = spawn_blocking_with_tracing(move || {
                let salt_string = salt();
                return Password::new(password, &salt_string);
            })
            .await
            .context("Failed to spawn tokio blocking task to rehash outdated password")?
            .context("Failed hash the password of user")?;

            transaction.update_user_password(user_id.clone(), hash_result)
                .await
                .context("Failed to password of user in Postgres")?;
        }
    };

    let new_session = UserSession::<NewlyCreated>::new(user_id.into());
    transaction.save_newly_created_user_session(&new_session)
        .await
        .context("Failed to save new user session to the database")?;

    let cipher = state.new_token_encryptor();
    let (encrypted_refresh_token, encrypted_access_token) =
        spawn_blocking_with_tracing(move || {
            let encrypted_refresh_token = cipher.encrypt(new_session.state().refresh_token());

            let encrypted_access_token = cipher.encrypt(new_session.state().access_token());

            (encrypted_refresh_token, encrypted_access_token)
        })
        .await
        .context("Failed to spawn blocking tokio task to encrypt session tokens")?;

    let encrypted_refresh_token =
        encrypted_refresh_token.context("Failed to encrypt refresh token")?;
    let encrypted_access_token =
        encrypted_access_token.context("Failed to encrypt access token")?;

    transaction
        .commit()
        .await
        .context("Failed to commit transaction")?;

    Ok(LoginResponse::UserLoggedInSuccessfully {
        // we are removing 30 seconds from the actual expiration time, to increase the
        // likelihood of the token refreshing the tokens on time
        access_token: encrypted_access_token.token.expose_secret().clone(),
        access_token_expiration: encrypted_access_token.expires_at - Duration::seconds(30),
        refresh_token: encrypted_refresh_token.token.expose_secret().clone(),
        refresh_token_expiration: encrypted_refresh_token.expires_at - Duration::seconds(30),
    })
}

fn get_dummy_hash() -> anyhow::Result<Password> {
    Ok(Password::try_from("$argon2id$v=19$m=15000,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno")?)
}

#[tracing::instrument(
    name = "Comparing expected password with submitted password",
    skip(expected_password, submitted_password)
)]
async fn verify_if_password_matches(
    expected_password: Password,
    submitted_password: &Secret<String>,
) -> Result<Result<MatchResult, MatchError>, JoinError> {
    // Moving matches into a different thread because its operation is considered heavy,
    // which can block tokio runtime.
    let password = submitted_password.clone();
    spawn_blocking_with_tracing(move || expected_password.matches(&password)).await
}

fn salt() -> SaltString {
    SaltString::generate(&mut rand::thread_rng())
}