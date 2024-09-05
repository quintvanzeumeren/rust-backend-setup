use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::policy::policies::create_team_policy::CreateTeamPolicy;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyRejectionError;

#[tracing::instrument(
    name = "Adding a new team"
    skip_all,
)]
pub async fn create_team(user: UserWithPolicy<CreateTeamPolicy>, new_team_request: Json<NewTeamRequestBody>) -> Result<StatusCode, PolicyRejectionError> {
    let create_team_contract = user.policy.authorize(()).await?;
    let response = create_team_contract.create_team(new_team_request.team_id.into()).await;
    Ok(match response {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(Deserialize, Clone)]
pub struct NewTeamRequestBody {
    pub team_id: Uuid
}
