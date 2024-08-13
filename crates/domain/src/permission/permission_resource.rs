use std::collections::HashSet;
use std::fs::Permissions;
use crate::permission::permission::PermissionName;
use crate::permission::resource::resource_id::ResourceId;
use crate::permission::resource_type::ResourceType;

// pub enum PermissionResource {
//     WithoutResource(PermissionName),
//     WithTeamResources {
//         permissions: Permissions,
//         resource_id: HashSet<ResourceId>,
//     }
// }
// impl PermissionResource {
//     pub fn resource_type(&self) -> Option<&'static str> {
//         match self {
//             PermissionResource::WithoutResource(_) => None,
//             PermissionResource::WithTeamResources { .. } => Some(ResourceType::Team.into())
//         }
//     }
// }
