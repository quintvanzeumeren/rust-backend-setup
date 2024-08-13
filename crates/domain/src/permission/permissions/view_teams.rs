use std::collections::HashSet;
use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserAttributes;
use crate::team::team_id::TeamId;

pub struct ViewTeam {
    pub user_attributes: UserAttributes,
    pub viewable_teams: HashSet<TeamId>
}

impl Permission for ViewTeam {
    type ResourceInQuestion = TeamId;

    fn name() -> PermissionName {
        "ViewTeam"
    }

    fn is_authorized_for(&self, team_id: <Self as Permission>::ResourceInQuestion) -> bool {
        self.user_attributes.is_root() 
            || self.user_attributes.teams.contains(&team_id)
            || self.viewable_teams.contains(&team_id)
    }
}