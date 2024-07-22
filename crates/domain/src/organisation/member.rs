use uuid::Uuid;

use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission::Permission;
use crate::user::user_id::UserId;

pub struct Member<P: Permission> {
    pub user_id: UserId,
    pub organisation_id: OrganisationId,
    pub permission: P
}

pub struct MemberPermissions {
    pub user_id: UserId,
    pub organisation_id: OrganisationId,
    pub member_id: Uuid
    
}