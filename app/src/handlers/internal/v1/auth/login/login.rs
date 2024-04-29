use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use password_hash::SaltString;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, Postgres, query, Transaction};
use tokio::task::JoinError;
use uuid::Uuid;
use lib_auth::security::encryption::encryptor::Encryptor;
use lib_auth::security::token::token_encryptor::EncryptedToken;

use lib_domain::sessions::state::newly_created::NewlyCreated;
use lib_domain::sessions::user_session::UserSession;
use lib_domain::user::password::{MatchError, MatchResult, Password};
use crate::app_state::AppState;

use crate::handlers::internal::v1::auth::authentication_error::{AuthenticationError, AuthenticationResult};
use crate::queries::save_newly_created_user_session::save_newly_created_user_session;
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

// TODO: move get_user_credentials, and update_user_password out of this module and into the queries module.

#[tracing::instrument(
    name = "Logging with username and hash",
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
    let mut expected_user_password = get_dummy_hash()
        .context("Failed to create default password")?;

    let optional_user_credentials = get_user_credentials(&state.db, &credentials.username)
        .await
        .context("Failed to get user credentials from Postgres")?;

    if let Some(user_credentials) = optional_user_credentials {
        user_id = Some(user_credentials.user_id);
        expected_user_password = user_credentials.password;
    }

    let match_result = verify_if_password_matches(expected_user_password, &credentials.password)
        .await
        .context("Failed to spawn blocking tokio task to verify password")?
        .context("Failed to submitted password with expected password")?;

    let user_id = user_id.ok_or(AuthenticationError::CredentialsInvalid)?;
    let mut transaction = state.db.begin().await
        .context("Failed to start a Postgres transaction")?;

    match match_result  {
        MatchResult::DoesNotMatch => return Err(AuthenticationError::CredentialsInvalid),
        MatchResult::Matches => {},
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

            update_user_password(&mut transaction, user_id.clone(), hash_result)
                .await
                .context("Failed to password of user in Postgres")?;
        }
    };

    let new_session = UserSession::<NewlyCreated>::new(&user_id);
    save_newly_created_user_session(&mut transaction, &new_session).await
        .context("Failed to save new user session to the database")?;

    let cipher = state.new_token_encryptor();
    let (encrypted_refresh_token, encrypted_access_token) =
        spawn_blocking_with_tracing(move || {
            let encrypted_refresh_token =
                cipher.encrypt(new_session.state().refresh_token());

            let encrypted_access_token =
                cipher.encrypt(new_session.state().access_token());

            (encrypted_refresh_token, encrypted_access_token)
        })
        .await
        .context("Failed to spawn blocking tokio task to encrypt session tokens")?;

    let encrypted_refresh_token = encrypted_refresh_token.context("Failed to encrypt refresh token")?;
    let encrypted_access_token = encrypted_access_token.context("Failed to encrypt access token")?;

    transaction.commit().await.context("Failed to commit transaction")?;

    Ok(LoginResponse::UserLoggedInSuccessfully {
        access_token: encrypted_access_token.token.expose_secret().clone(),
        access_token_expiration: encrypted_access_token.expires_at,
        refresh_token: encrypted_refresh_token.token.expose_secret().clone(),
        refresh_token_expiration: encrypted_refresh_token.expires_at,
    })
}

fn get_dummy_hash() -> anyhow::Result<Password> {
    Ok(Password::try_from("$argon2id$v=19$m=15000,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno")?)
}

struct UserCredentials {
    user_id: Uuid,
    password: Password,
}

#[tracing::instrument(
    name = "Fetching user credentials for username",
    skip(db, username)
)]
async fn get_user_credentials(db: &PgPool, username: &String) -> anyhow::Result<Option<UserCredentials>> {
    let row = query!(
        r#"
           SELECT user_id, password_hash FROM users
           WHERE username = $1
        "#,
        username
    )
        .fetch_optional(db)
        .await
        .context("Failed to perform a query to retrieve stored credentials.")?
        .map(|row| (row.user_id, Secret::new(row.password_hash)));

    if row.is_none() {
        return Ok(None)
    }

    let (user_id, pw_hash) = row.unwrap();
    let password = Password::try_from(pw_hash.expose_secret().clone())?;

    Ok(Some(
        UserCredentials {
            user_id,
            password,
        }
    ))
}

#[tracing::instrument(
    name = "Comparing expected password with submitted password",
    skip(expected_password, submitted_password)
)]
async fn verify_if_password_matches(expected_password: Password, submitted_password: &Secret<String>) -> Result<Result<MatchResult, MatchError>, JoinError> {
    // Moving matches into a different thread because its operation is considered heavy,
    // which can block tokio runtime.
    let password =  submitted_password.clone();
    spawn_blocking_with_tracing(move || expected_password.matches(&password)).await
}

fn salt() -> SaltString {
    SaltString::generate(&mut rand::thread_rng())
}

#[tracing::instrument(
    name = "Saving updated password of user to Postgres",
    skip(transaction, user_id, new_password)
)]
async fn update_user_password(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    new_password: Password
) -> Result<(), sqlx::Error> {
    let password_hash = new_password.hash_string();
    let query = sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE user_id = $2;
        "#,
        password_hash.expose_secret(),
        user_id
    );

    transaction.execute(query).await?;
    return Ok(());

}

// type EncryptSessionTokenResult = Result<EncryptedToken, LocalPasetoV4EncryptionError>;

// async fn encrypt_session_tokens(
//     session: UserSession<NewlyCreated>,
//     encryption_key: SymmetricKey<V4>
// ) -> Result<(EncryptSessionTokenResult, EncryptSessionTokenResult), JoinError> {
//     spawn_blocking_with_tracing(move || {
//         let encrypted_refresh_token =
//             session.state().refresh_token().encrypt(&encryption_key);
// 
//         let encrypted_access_token =
//             session.state().access_token().encrypt(&encryption_key);
// 
//         (encrypted_refresh_token, encrypted_access_token)
//     }).await
// }
