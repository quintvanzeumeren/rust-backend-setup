use std::collections::HashSet;

use crate::team::team_id::TeamId;
use crate::permission::permission::{Permission, PermissionName};
use crate::permission::resource::resource::Resource;
use crate::permission::user_attributes::UserAttributes;

/// ReadTeamMembers checks if the user can read the members of an team
pub struct ReadTeamMembers {
    pub user_attributes: UserAttributes,
    pub resources: HashSet<Resource<TeamId>>
}

impl Permission for ReadTeamMembers {
    type ResourceInQuestion = TeamId;

    fn name() -> PermissionName {
        "ReadTeamMembers"
    }

    fn is_authorized_for(&self, team_id: <Self as Permission>::ResourceInQuestion) -> bool {
        if self.user_attributes.is_root() {  
            return true;
        }
        
        let user_is_part_of_team = self.user_attributes.teams.contains(&team_id);
        if user_is_part_of_team {
            return true;
        }

        return self.resources.iter()
            .map(|r| r.resource)
            .any(|r| r == team_id);
    }
}

