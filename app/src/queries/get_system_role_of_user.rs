use serde::{Deserialize, Serialize};
use sqlx::{query_file, query_file_as};
use domain::role::role::SystemRole;
use domain::user::user_id::UserId;
use crate::queries::database::Database;

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
    pub system_role: Option<SystemRole>
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "system_role")]
pub enum SystemRoleType {
    Root,
    Admin
}