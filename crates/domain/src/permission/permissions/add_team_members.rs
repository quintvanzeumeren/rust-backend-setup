use std::collections::HashSet;

use crate::team::team_id::TeamId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};

/// AddTeamMembers checks if a user can add another user to as team
pub struct AddTeamMembers {
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