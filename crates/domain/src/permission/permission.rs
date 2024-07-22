use std::collections::HashSet;
use std::fs::Permissions;
use crate::permission::permission_authorizer::PermissionName;
use crate::permission::resource::resource_id::ResourceId;
use crate::permission::resource_type::ResourceType;

pub enum Permission {
    WithoutResource(PermissionName),
    WithOrganisationResources {
        permissions: Permissions,
        resource_id: HashSet<ResourceId>,
    }
}
impl Permission {
    pub fn resource_type(&self) -> Option<&'static str> {
        match self {
            Permission::WithoutResource(_) => None,
            Permission::WithOrganisationResources { .. } => Some(ResourceType::Organisation.into())
        }
    }
}
