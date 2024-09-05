use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::role::role::SystemRole;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRoleRecord {
    pub user_id: Uuid,
    pub team_id: Option<Uuid>,
    pub role: SystemRoleType,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "system_role")]
pub enum SystemRoleType {
    Root,
    Admin
}

impl From<SystemRole> for SystemRoleType {
    fn from(value: SystemRole) -> Self {
        match value {
            SystemRole::Root => SystemRoleType::Root,
            SystemRole::Admin => SystemRoleType::Admin
        }
    }
}
