use std::collections::HashSet;
use anyhow::Context;
use axum::Json;
use uuid::Uuid;
use domain::team::team_id::TeamId;
use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::{HandlerError, HandlerResponse};
use crate::policy::policies::get_teams_policy::GetTeamsPolicy;
use crate::policy::policy::Policy;

pub async fn get_teams(user: UserWithPolicy<GetTeamsPolicy>) -> HandlerResponse<Json<HashSet<Uuid>>> {
    let teams_contract = user.policy.authorize(()).await?;
    
    let teams = teams_contract.get_teams()
        .await
        .context("Failed to get teams for users")?
        .iter()
        .map(|t| t.0)
        .collect();

    Ok(Json(teams))
}


