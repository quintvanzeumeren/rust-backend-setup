use std::collections::HashSet;

use crate::team::team_id::TeamId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
use crate::permission::resource::resource::Resource;
use crate::permission::user_details::UserAttributes;

/// ReadTeamMembers checks if the user can read the members of an team
pub struct ReadTeamMembers {
    pub user_attributes: UserAttributes,
    pub resources: HashSet<Resource<TeamId>>
}

impl PermissionAuthorizer for ReadTeamMembers {
    type ResourceInQuestion = ReadTeamMembersResource;

    fn name() -> PermissionName {
        "ReadTeamMembers"
    }

    fn is_authorized_for(&self, context: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        let user_is_part_of_team = self.user_attributes.teams.contains(&context.team_id);
        if user_is_part_of_team {
            return true;
        }

        let user_has_access_to_resource = self.resources.iter()
            .map(|r| r.resource)
            .any(|r| r == context.team_id);

        return user_has_access_to_resource;
    }
}

pub struct ReadTeamMembersResource {
    pub team_id: TeamId
}

