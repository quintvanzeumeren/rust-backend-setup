use std::collections::HashSet;

use crate::team::team_id::TeamId;
use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserAttributes;
use crate::role::role::ROLE_ROOT;

/// AddTeamMembers checks if a user can add another user to as team
pub struct AddTeamMembers {
    pub user: UserAttributes,
    pub teams_where_users_can_be_added_to: HashSet<TeamId>
}

impl Permission for AddTeamMembers {
    type ResourceInQuestion = TeamId;

    fn name() -> PermissionName {
        "AddTeamMembers"
    }

    fn is_authorized_for(&self, team_id: <Self as Permission>::ResourceInQuestion) -> bool {
        self.user.is_root() || self.teams_where_users_can_be_added_to.contains(&team_id)
    }
}