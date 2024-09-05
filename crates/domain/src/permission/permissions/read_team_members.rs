use crate::permission::permission::{Permission, PermissionName};
use crate::user::user_details::UserDetails;
use crate::team::team_id::TeamId;

/// ReadTeamMembers checks if the user can read the members of an team
pub struct ReadTeamMembers {
    pub user_attributes: UserDetails,
}

impl ReadTeamMembers {
    pub fn new(user_attributes: UserDetails) -> Self {
        Self { user_attributes }
    }
}

impl Permission for ReadTeamMembers {
    type Details = TeamId;

    fn name() -> PermissionName {
        "ReadTeamMembers"
    }

    fn is_authorized_for(&self, team_id: <Self as Permission>::Details) -> bool {
        if self.user_attributes.is_root_or_admin() {  
            return true;
        }
        
        self.user_attributes.teams.contains(&team_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::permission::permission::Permission;
    use crate::permission::permissions::read_team_members::ReadTeamMembers;
    use crate::user::user_details::tests::{random_user_attributes_admin, random_user_attributes_root, random_user_attributes_with};
    use uuid::Uuid;

    #[test]
    fn test_read_members_name() {
        assert_eq!(ReadTeamMembers::name(), "ReadTeamMembers");
    }

    #[test] 
    fn test_read_team_members_authorization() {
        let team_id = Uuid::new_v4().into();
        let admin = random_user_attributes_admin(vec![]);
        let permission = ReadTeamMembers::new(admin);
        assert!(permission.is_authorized_for(team_id));

        let root = random_user_attributes_root(vec![]);
        let permission = ReadTeamMembers::new(root);
        assert!(permission.is_authorized_for(team_id));

        let member = random_user_attributes_with(vec![team_id.0.clone()], vec![]);
        let permission = ReadTeamMembers::new(member);
        assert!(permission.is_authorized_for(team_id));
        
        let non_member = random_user_attributes_with(vec![], vec![]);
        let permission = ReadTeamMembers::new(non_member);
        assert!(!permission.is_authorized_for(team_id));
    }
}
