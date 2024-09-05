use std::collections::HashSet;
use anyhow::Context;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::role::role::SystemRole;
use domain::team::membership::Membership;
use domain::user::user_details::UserDetails;
use domain::user::user_id::UserId;
use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::{HandlerError, HandlerResponse};
use crate::policy::policies::read_user_details_policy::ReadUserDetailsPolicy;
use crate::policy::policy::Policy;

#[derive(Deserialize)]
pub struct UserParams {
    user_id: UserId
}

#[derive(Serialize)]
pub struct UserDetailsResponse {
    id: UserId,
    teams: HashSet<Membership>,
    system_role: Option<SystemRole>
}

impl From<UserDetails> for UserDetailsResponse {
    fn from(user: UserDetails) -> Self {
        Self {
            id: user.id,
            teams: user.teams,
            system_role: user.system_role,
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