use axum::Json;
use reqwest::StatusCode;
use secrecy::Secret;
use serde::Deserialize;
use uuid::Uuid;
use domain::role::role_name::RoleName;
use domain::shared::slug::Slug;
use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::HandlerResponse;
use crate::policy::policies::create_user_policy::CreateUserPolicy;
use crate::policy::policy::Policy;

#[derive(Deserialize)]
pub struct AddUserBody {
    id: Uuid,
    username: String,
    password: Secret<String>,
    roles: Vec<String>
}

pub async fn add_user(user: UserWithPolicy<CreateUserPolicy>, Json(new_user): Json<AddUserBody>) -> HandlerResponse<()> {
    user.policy.authorize(
        new_user
            .roles
            .iter()
            .map(|r| RoleName(Slug::from(r.clone())))
            .collect()
    )?;
    
    todo!()
}