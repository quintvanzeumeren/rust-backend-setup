use crate::role::role::{SystemRole};
use crate::team::membership::Membership;
use crate::user::user_id::UserId;
use std::collections::HashSet;

/// UserAttributes contains attributes of the User by which a Permission can determine if the user
/// is authorized.
#[derive(Debug, Clone)]
pub struct UserDetails {
    pub id: UserId,
    pub teams: HashSet<Membership>,
    pub system_role: Option<SystemRole>
}