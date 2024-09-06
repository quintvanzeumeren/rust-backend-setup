use crate::queries::database::Database;
use domain::team::member::Member;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use serde::{Deserialize, Serialize};
use sqlx::query_file;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct TeamMemberRecord {
    user_id: UserId,
    team_id: TeamId,
    manager: bool
}

impl Database {
    
    pub async fn get_members_by_team_id(&self, team_id: TeamId) -> sqlx::Result<HashSet<Member>> {
        let members = query_file!(
            "src/queries/get_members_by_team_id.sql",
            team_id.0
        ).fetch_all(self.db()).await?;
        
        Ok(members.iter().map(|r| Member {
            user_id: r.user_id.into(),
            team_id,
            manager: r.manager,
        }).collect())
    }
}