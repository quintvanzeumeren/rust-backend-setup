use std::collections::HashSet;
use sqlx::query_file;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;
use crate::queries::database::Database;

impl Database {
    
     pub async fn get_user_teams(&self, user_id: UserId) -> sqlx::Result<UserTeams> {
        let results = query_file!(
            "src/queries/get_user_teams.sql",
            user_id.0,
        ).fetch_all(self.db()).await?;

         let teams: HashSet<TeamId> = results.iter().map(|r| TeamId::from(r.team_id)).collect();
        
        Ok(UserTeams {
            user_id,
            teams,
        })
    }
}

pub struct UserTeams {
    pub user_id: UserId,
    pub teams: HashSet<TeamId>
}