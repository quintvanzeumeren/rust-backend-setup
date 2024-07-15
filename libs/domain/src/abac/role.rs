use uuid::Uuid;
use crate::abac::permission::PermissionName;
use crate::organisation::organisation::Organisation;
use crate::organisation::organisation_id::OrganisationId;
use crate::user::user::User;
use crate::user::user_id::UserId;

pub struct Role {
    id: Uuid,
    organisation_id: OrganisationId,
    name: String,
    permissions: Vec<PermissionDetails>
}

pub struct PermissionDetails {
    id: Uuid,
    resource: Option<Resource>,
    permission_name: PermissionName
}

pub struct Resource {
    id: Uuid,
    resource_type: ResourceType,
}

enum ResourceType {
    User(UserId),
    Organisation(OrganisationId)
}