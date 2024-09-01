use std::collections::HashSet;
use sqlx::query_file;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::queries::database::Database;

impl Database {
    
    pub async fn get_team_members(&self, team_id: TeamId) -> sqlx::Result<HashSet<UserId>> {
        let result = query_file!(
            "src/queries/get_members_for_team.sql",
            team_id.0
        ).fetch_all(self.db()).await?;
        
        Ok(result.iter().map(|r| r.user_id.into()).collect())
    }
}