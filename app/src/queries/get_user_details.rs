use std::collections::HashSet;
use sqlx::{Error, query_file, query_file_as};
use tokio::try_join;
use domain::permission::user_attributes::UserDetails;
use domain::user::user_id::UserId;

use crate::queries::database::Database;
use crate::queries::get_system_role_of_user::UserSystemRole;

impl Database {
    #[tracing::instrument(name = "Fetching user details for user id", skip(self))]
    pub async fn get_user_details(&self, user_id: UserId) -> sqlx::Result<Option<UserDetails>> {
        let (user, memberships) = try_join!(
            self.get_system_role_of_user(user_id),
            self.get_user_memberships(user_id)
        )?;

        // Checks if user actually exist.
        let user = match user {
            None => return Ok(None),
            Some(user) => user
        };

        Ok(Some(UserDetails {
            id: user_id,
            teams: memberships,
            system_role: user.system_role,
        }))
    }
}
