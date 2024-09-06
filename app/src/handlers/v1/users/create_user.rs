use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::HandlerResponse;
use crate::policy::policies::create_user_policy::{CreateUserDetails, CreateUserPolicy};
use crate::policy::policy::Policy;
use crate::telemetry::spawn_blocking_with_tracing;
use anyhow::Context;
use axum::http::StatusCode;
use axum::Json;
use domain::role::role::{SystemRole};
use domain::user::password::Password;
use domain::user::user_credentials::UserCredentials;
use domain::user::user_id::UserId;
use password_hash::SaltString;
use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequestBody {
    id: UserId,
    username: String,
    password: Secret<String>,
    role: Option<SystemRole>
}

pub async fn create_user(user: UserWithPolicy<CreateUserPolicy>, Json(new_user): Json<CreateUserRequestBody>) -> HandlerResponse<StatusCode> {
    // authorize logged in user to see if it can create the user with the given roles
    let new_user_contract = user.policy.authorize(CreateUserDetails {
        role: new_user.role,
        team_to_part_of: None,
    }).await?;
    
    // hash password of new user
    let password = new_user.password;
    let hashed_pw = spawn_blocking_with_tracing(move || {
        let salt_string = SaltString::generate(&mut rand::thread_rng());
        return Password::new(password, &salt_string);
    })
        .await
        .context("Failed to spawn tokio blocking task to hash password")?
        .context("Failed hash the password of user")?;
    
    // transform new user into a user struct and save it via the contract.
    let user = UserCredentials {
        id: new_user.id.into(),
        username: new_user.username,
        password: hashed_pw
    };

    new_user_contract
        .create_user(user)
        .await
        .context("Failed to create new user")?;
    
    Ok(StatusCode::CREATED)
}