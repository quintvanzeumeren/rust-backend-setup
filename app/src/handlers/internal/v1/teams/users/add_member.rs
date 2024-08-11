use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use domain::permission::permissions::add_team_members::{AddTeamMembers, AddUserToTeamContext};
use crate::extractors::user::permission_extractor::permission_of::PermissionOf;
use crate::extractors::user::user_with::{QueryParams, UserWith};

#[derive(Deserialize, Clone)]
pub struct AddMemberParams {
    pub team_id: Uuid,
    pub user_id: Uuid
}

impl Into<AddUserToTeamContext> for AddMemberParams {
    fn into(self) -> AddUserToTeamContext {
        AddUserToTeamContext {
            team_to_gain_user: self.team_id.into()
        }
    }
}

pub async fn add_member(user: UserWith<PermissionOf<AddTeamMembers, AddMemberParams>, QueryParams>) -> StatusCode {
    todo!("Add easier type for user to add functionality");
    todo!("Implement handler...")
}