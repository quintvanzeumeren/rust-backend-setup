use anyhow::Context;
use axum::extract::Path;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::HandlerResponse;
use crate::policy::policies::add_team_members_policy::AddTeamMembersPolicy;
use crate::policy::policy::Policy;
use crate::telemetry::TelemetryRecord;

#[derive(Deserialize, Clone)]
pub struct AddMemberParams {
    pub team_id: Uuid,
    pub user_id: Uuid
}

#[tracing::instrument(
    name = "Adding new user to team",
    skip(user, params), 
    fields (
        new_member_id = tracing::field::Empty,
        team_id = tracing::field::Empty,
    )
)]
pub async fn add_member(user: UserWithPolicy<AddTeamMembersPolicy>, Path(params): Path<AddMemberParams>) -> HandlerResponse<StatusCode> {
    params.user_id.record_in_telemetry("new_member_id");
    params.team_id.record_in_telemetry("team_id");
    
    let add_members_contract = user.policy.authorize(params.team_id.into())?;
    add_members_contract.add_member(params.user_id.into())
        .await
        .context("Failed to add member to team")?;
    
    Ok(StatusCode::OK)
}