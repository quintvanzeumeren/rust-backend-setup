use std::collections::HashSet;

use crate::team::team_id::TeamId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
use crate::permission::user_attributes::UserAttributes;

/// DeleteTeam is a permission that checks if whenever the user can delete an team.
pub struct DeleteTeam {
    pub user_attributes: UserAttributes,
    pub deletable_teams: HashSet<TeamId>
}

impl PermissionAuthorizer for DeleteTeam {
    type ResourceInQuestion = DeleteTeamContext;

    fn name() -> PermissionName {
        "DeleteTeam"
    }

    fn is_authorized_for(&self, resource_in_question: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        // todo probably add more conditions to this delete operation
        // like cannot delete team of paying customers..
        let part_of_team = self.user_attributes.teams.contains(&resource_in_question.team_to_delete);
        if part_of_team {
            return false
        }

        self.deletable_teams.contains(&resource_in_question.team_to_delete)
    }
}

pub struct DeleteTeamContext {
    pub team_to_delete: TeamId
}