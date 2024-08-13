use std::collections::HashSet;

// use crate::permission::permission::PermissionResource;
use crate::role::role_id::RoleId;
use crate::role::role_name::RoleName;

pub const ROLE_ROOT: &'static str = "root";

pub struct Role {
    pub id: RoleId,
    pub name: RoleName,
    // pub permissions: HashSet<PermissionResource>
}