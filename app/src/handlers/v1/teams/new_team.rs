use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use domain::permission::permission::Permission;
use domain::permission::permissions::create_team::CreateTeam;

use crate::extractors::user::user_with_policy::UserWithPolicy;
use crate::policy::policies::create_team_policy::CreateTeamPolicy;
use crate::policy::policy::Policy;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

#[tracing::instrument(
    name = "Adding a new team"
    skip_all,
)]
pub async fn new_team(user: UserWithPolicy<CreateTeamPolicy>, new_team_request: Json<NewTeamRequestBody>) -> Result<StatusCode, PolicyAuthorizationError> {
    let create_team_contract = user.policy.authorize(())?;
    let response = create_team_contract.create_team(new_team_request.team_id.into()).await;
    return Ok(match response {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(Deserialize, Clone)]
pub struct NewTeamRequestBody {
    pub team_id: Uuid
}

impl Into<()> for NewTeamRequestBody {
    fn into(self) -> <CreateTeam as Permission>::Details {
        ()
    }
}
