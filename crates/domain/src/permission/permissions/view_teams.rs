use std::collections::HashSet;
use crate::permission::permission::{Permission, PermissionName};
use crate::user::user_details::UserDetails;
use crate::team::team_id::TeamId;

pub struct ViewTeam {
    pub user_attributes: UserDetails,
}

impl ViewTeam {
    pub fn new(user_attributes: UserDetails) -> Self {
        Self { user_attributes }
    }
}

impl Permission for ViewTeam {
    type Details = TeamId;

    fn name() -> PermissionName {
        "ViewTeam"
    }

    fn is_authorized_for(&self, team_id: <Self as Permission>::Details) -> bool {
        self.user_attributes.is_root_or_admin() || self.user_attributes.teams.contains(&team_id)
    }
}


#[cfg(test)]
mod tests {
    extern crate test_utility;

    use uuid::Uuid;
    use crate::permission::permission::Permission;
    use crate::permission::permissions::view_teams::ViewTeam;
    use crate::user::user_details::tests::{random_user_attributes_admin, random_user_attributes_root, random_user_attributes_with};
    use crate::team::team_id::TeamId;

    #[test]
    fn test_correct_permission_name() {
        assert_eq!(ViewTeam::name(), "ViewTeam")
    }
    
    #[test]
    fn test_authorized_for() {
        // test when root user
        let user = random_user_attributes_root(Default::default());
        let p = ViewTeam::new(user);
        let team_id = Uuid::new_v4().into();
        assert!(p.is_authorized_for(team_id));

        // test when admin user
        let user = random_user_attributes_admin(Default::default());
        let p = ViewTeam::new(user);
        let team_id = Uuid::new_v4().into();
        assert!(p.is_authorized_for(team_id));

        // test when part of team
        let team_id: TeamId = Uuid::new_v4().into();
        let user = random_user_attributes_with(vec![team_id.0], vec![]);
        let p = ViewTeam::new(user);
        assert!(p.is_authorized_for(team_id));
        
        // test when not part of team
        let team_id = Uuid::new_v4().into();
        assert!(!p.is_authorized_for(team_id));
    }
}