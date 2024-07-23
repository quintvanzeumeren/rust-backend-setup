use std::collections::HashSet;

use crate::team::team_id::TeamId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
use crate::permission::user_attributes::UserAttributes;
use crate::role::role::ROLE_ROOT;

/// AddTeamMembers checks if a user can add another user to as team
pub struct AddTeamMembers {
    pub user: UserAttributes,
    pub teams_where_users_can_be_added_to: HashSet<TeamId>
}

impl PermissionAuthorizer for AddTeamMembers {
    type ResourceInQuestion = AddUserToTeamContext;

    fn name() -> PermissionName {
        "AddTeamMembers"
    }

    fn is_authorized_for(&self, context: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        self.teams_where_users_can_be_added_to.contains(&context.team_to_gain_user)
    }
}

pub struct AddUserToTeamContext {
    team_to_gain_user: TeamId
}