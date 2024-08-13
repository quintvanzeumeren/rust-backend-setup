use axum::http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use crate::extractors::user::user_with::UserWith;
use crate::policy::policies::add_team_members_policy::AddTeamMembersPolicy;

#[derive(Deserialize, Clone)]
pub struct AddMemberParams {
    pub team_id: Uuid,
    pub user_id: Uuid
}

pub async fn add_member(user: UserWith<AddTeamMembersPolicy>) -> StatusCode {
    todo!("Add easier type for user to add functionality");
    // todo!("Implement handler...")
    
    // let team = user.add_members_to_team()?;
    // team.add_member(user_id);
    // 
    // user.add_members_to_team
    
}