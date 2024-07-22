use std::collections::HashSet;

use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission::Permission;
use crate::role::role_id::RoleId;
use crate::shared::slug::Slug;

pub struct Role {
    pub id: RoleId,
    pub organisation_id: OrganisationId,
    pub name: Slug,
    pub permissions: HashSet<Permission>
}