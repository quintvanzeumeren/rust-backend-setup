use std::collections::HashSet;

use crate::permission::permission::Permission;
use crate::role::role_id::RoleId;
use crate::shared::slug::Slug;

pub struct Role {
    pub id: RoleId,
    pub name: Slug,
    pub permissions: HashSet<Permission>
}