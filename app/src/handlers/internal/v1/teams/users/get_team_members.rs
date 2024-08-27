use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::HandlerResponse;
use crate::policy::policies::get_team_members_policy::GetTeamMembersPolicy;
use crate::policy::policy::Policy;
use anyhow::Context;
use axum::extract::Path;
use axum::Json;
use serde::Deserialize;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetTeamMemberParams {
    team_id: Uuid
}

pub async fn get_team_members(user: UserWithPolicy<GetTeamMembersPolicy>, Path(params): Path<GetTeamMemberParams>) -> HandlerResponse<Json<HashSet<Uuid>>> {
    let get_members_contract = user.policy.authorize(params.team_id.into())?;
    let members = get_members_contract.fetch_team_members()
        .await
        .context("Failed to fetch team members")?;
    
    Ok(Json(members.iter().map(|id| id.0).collect()))
}