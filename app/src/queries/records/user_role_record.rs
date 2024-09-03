use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRoleRecord {
    pub user_id: Uuid,
    pub team_id: Option<Uuid>,
    pub role: RoleName,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug, Hash, PartialEq, Eq, Clone)]
#[sqlx(type_name = "user_role")]
pub enum RoleName {
    Root,
    Admin,
    TeamManager,
    Member
}

impl Display for RoleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RoleName::Root => "Root".to_string(),
            RoleName::Admin => "Admin".to_string(),
            RoleName::TeamManager => "TeamManager".to_string(),
            RoleName::Member => "Member".to_string(),
        };
        write!(f, "{}", str)
    }
}
