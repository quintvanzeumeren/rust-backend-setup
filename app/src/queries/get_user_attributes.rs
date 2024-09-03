use std::collections::HashSet;
use sqlx::{Error, query_file, query_file_as};
use tokio::try_join;
use domain::permission::user_attributes::UserDetails;
use domain::user::user_id::UserId;

use crate::queries::database::Database;

impl Database {
    #[tracing::instrument(name = "Fetching user details for user id", skip(self))]
    pub async fn get_user_details(&self, user_id: UserId) -> sqlx::Result<UserDetails> {
        let (user_roles, user_teams) = try_join!(
            self.get_user_roles(user_id),
            self.get_user_teams(user_id)
        )?;
        
        todo!("Fix user role mapping");
        Ok(UserDetails {
            id: user_id,
            teams: user_teams.teams,
            // roles: user_roles.roles.iter().map(|t| t.name.clone().into()).collect(),
            roles: HashSet::new()
        })
    }
}
