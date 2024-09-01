use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserAttributes;
use crate::team::team_id::TeamId;

/// AddTeamMembers checks if a user can add another user to as team
pub struct AddTeamMembers {
    pub user: UserAttributes,
}

impl AddTeamMembers {
    pub fn new(user: UserAttributes) -> Self {
        Self { user }
    }
}

impl Permission for AddTeamMembers {
    type Details = TeamId;

    fn name() -> PermissionName {
        "AddTeamMembers"
    }

    fn is_authorized_for(&self, _: <Self as Permission>::Details) -> bool {
        self.user.is_root_or_admin()
    }
}

#[cfg(test)]
mod tests {
    use crate::permission::permission::Permission;
    use crate::permission::permissions::add_team_members::AddTeamMembers;
    use crate::permission::user_attributes::tests::{random_user_attributes_admin, random_user_attributes_root, random_user_attributes_with};
    use uuid::Uuid;

    #[test]
    fn test_add_members_name() {
        assert_eq!(AddTeamMembers::name(), "AddTeamMembers");
    }

    #[test]
    fn test_read_team_members_authorization() {
        let team_id = Uuid::new_v4().into();
        
        let root = random_user_attributes_root(vec![]);
        let permission = AddTeamMembers::new(root);
        assert!(permission.is_authorized_for(team_id));
        
        let admin = random_user_attributes_admin(vec![]);
        let permission = AddTeamMembers::new(admin);
        assert!(permission.is_authorized_for(team_id));

        let member = random_user_attributes_with(vec![team_id.0.clone()], vec![]);
        let permission = AddTeamMembers::new(member);
        assert!(!permission.is_authorized_for(team_id));
        assert!(!permission.is_authorized_for(Uuid::new_v4().into()));
        
        let non_member = random_user_attributes_with(vec![], vec![]);
        let permission = AddTeamMembers::new(non_member);
        assert!(!permission.is_authorized_for(team_id));
    }
}