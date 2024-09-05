use crate::queries::database::Database;
use crate::queries::records::user_role_record::SystemRoleType;
use domain::user::user_id::UserId;
use serde::{Deserialize, Serialize};
use sqlx::query_file_as;

impl Database {
    pub async fn get_system_role_of_user(&self, user_id: UserId) -> sqlx::Result<Option<UserSystemRole>> {
        let record = query_file_as!(
            UserSystemRole,
            "src/queries/get_system_role_of_user.sql",
            user_id.0
        ).fetch_optional(self.db()).await?;

        Ok(record)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserSystemRole {
    pub user_id: UserId,
    pub system_role: Option<SystemRoleType>
}

