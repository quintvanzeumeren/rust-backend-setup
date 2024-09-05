use std::collections::HashSet;
use crate::role::role::{SystemRole, UserRoles, ROLE_ADMIN, ROLE_ROOT};
use crate::shared::slug::Slug;
use crate::team::membership::Membership;
use crate::team::team_id::TeamId;
use crate::user::user_id::UserId;

/// UserAttributes contains attributes of the User by which a Permission can determine if the user
/// is authorized.
#[derive(Debug, Clone)]
pub struct UserDetails {
    pub id: UserId,
    pub teams: HashSet<Membership>,
    pub system_role: Option<SystemRole>
}