use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct RoleRecord {
    pub id: Uuid,
    pub name: String
}