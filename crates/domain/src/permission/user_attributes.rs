use std::collections::HashSet;
use crate::role::role::ROLE_ROOT;
use crate::shared::slug::Slug;
use crate::team::team_id::TeamId;
use crate::user::user_id::UserId;

/// UserAttributes contains attributes of the User by which a Permission can determine if the user
/// is authorized.
#[derive(Debug, Clone)]
pub struct UserAttributes {
    pub id: UserId,
    pub teams: HashSet<TeamId>,
    pub roles: HashSet<Slug>
}

impl UserAttributes {
    pub fn is_root(&self) -> bool{
        self.roles.contains(&ROLE_ROOT.into())
    }
}