use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use domain::permission::permission_authorizer::PermissionAuthorizer;
use domain::permission::permissions::create_team::CreateTeam;
use crate::extractors::user::permission_extractor::permission_of::PermissionOf;
use crate::extractors::user::user_with::UserWith;

#[tracing::instrument(
    name = "Adding a new team"
    skip_all,
)]
pub async fn new_team(user: UserWith<PermissionOf<CreateTeam, NewTeamRequestBody>>) -> StatusCode {
    let response = user.create_new_team(user.request_content.team_id.into()).await;
    return match response {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Deserialize, Clone)]
pub struct NewTeamRequestBody {
    pub team_id: Uuid
}

impl Into<()> for NewTeamRequestBody {
    fn into(self) -> <CreateTeam as PermissionAuthorizer>::ResourceInQuestion {
        ()
    }
}
