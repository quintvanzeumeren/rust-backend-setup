use std::collections::HashSet;
use anyhow::Context;
use axum::http::StatusCode;
use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::HandlerResponse;
use crate::policy::policies::create_user_policy::{CreateUserPolicy, NewUserDetails};
use crate::policy::policy::Policy;
use axum::Json;
use password_hash::SaltString;
use domain::role::role_name::RoleName;
use domain::shared::slug::Slug;
use domain::user::user::User;
use secrecy::Secret;
use serde::Deserialize;
use uuid::Uuid;
use domain::role::role::{Role, UserRoles};
use domain::user::password::Password;
use domain::user::user_id::UserId;
use crate::telemetry::spawn_blocking_with_tracing;

#[derive(Deserialize)]
pub struct CreateUserRequestBody {
    id: UserId,
    username: String,
    password: Secret<String>,
    roles: HashSet<Role>
}

pub async fn create_user(user: UserWithPolicy<CreateUserPolicy>, Json(new_user): Json<CreateUserRequestBody>) -> HandlerResponse<StatusCode> {
    // authorize logged in user to see if it can create the user with the given roles
    let new_user_contract = user.policy.authorize(new_user.roles).await?;
    
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
    let user = User {
        id: new_user.id.into(),
        username: new_user.username,
        password: hashed_pw
    };

    new_user_contract
        .create_user(NewUserDetails { user })
        .await
        .context("Failed to create new user")?;
    
    Ok(StatusCode::CREATED)
}