use anyhow::Context;
use axum::extract::Path;
use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::handlers::error::HandlerResponse;
use crate::policy::policies::add_team_members_policy::{AddTeamMemberDetails, AddTeamMemberPolicy};
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
pub async fn add_member(user: UserWithPolicy<AddTeamMemberPolicy>, Path(params): Path<AddMemberParams>) -> HandlerResponse<StatusCode> {
    params.user_id.record_in_telemetry("new_member_id");
    params.team_id.record_in_telemetry("team_id");
    
    // let details = AddTeamMemberDetails {
    //     user_to_add: params.user_id.into(),
    //     team_to_add_to: params.team_id.into(),
    // };
    
    let add_members_contract = user.policy.authorize(params.team_id.into()).await?;
    add_members_contract.add_member(params.user_id.into())
        .await
        .context("Failed to add member to team")?;
    
    Ok(StatusCode::OK)
}