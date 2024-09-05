use std::collections::HashSet;
use anyhow::Context;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::permission::user_details::UserDetails;
use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::{HandlerError, HandlerResponse};
use crate::policy::policies::read_user_details_policy::ReadUserDetailsPolicy;
use crate::policy::policy::Policy;

#[derive(Deserialize)]
pub struct UserParams {
    user_id: Uuid
}

#[derive(Serialize)]
pub struct UserDetailsResponse {
    id: Uuid,
    teams: HashSet<Uuid>,
    roles: HashSet<String>
}

impl From<UserDetails> for UserDetailsResponse {
    fn from(value: UserDetails) -> Self {
        Self {
            id: value.id.0,
            teams: value.teams.iter().map(|t| t.0).collect(),
            roles: value.roles.iter().map(|t| t.0.clone()).collect(),
        }
    }
}

pub async fn get_user_details(user: UserWithPolicy<ReadUserDetailsPolicy>, Path(params): Path<UserParams>) -> HandlerResponse<Json<UserDetailsResponse>> {
    let contract = user.policy.authorize(params.user_id.into()).await?;
    let user_attributes = contract.get_user_details()
        .await
        .context("Failed to get user details for user")?;
    
    if let Some(attributes) = user_attributes {
        return Ok(Json(attributes.into()))
    }
    
    Err(HandlerError::NotFound)
}