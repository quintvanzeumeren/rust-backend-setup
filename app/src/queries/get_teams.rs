use std::collections::HashSet;
use sqlx::query_file;
use domain::team::team_id::TeamId;
use crate::queries::database::Database;

impl Database {
    
    pub async fn get_teams(&self) -> sqlx::Result<HashSet<TeamId>> {
        let teams = query_file!("src/queries/get_teams.sql")
            .fetch_all(self.db())
            .await?;
        
        Ok(teams.iter().map(|r| r.id.into()).collect())
    }
}