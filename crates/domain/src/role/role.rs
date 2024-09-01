use crate::role::role_id::RoleId;
use crate::role::role_name::RoleName;

pub const ROLE_ROOT: &'static str = "root";
pub const ROLE_ADMIN: &'static str = "admin";

pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
}