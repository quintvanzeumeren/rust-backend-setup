use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserDetails;
use crate::team::team_id::TeamId;

/// DeleteTeam is a permission that checks if whenever the user can delete an team.
pub struct DeleteTeam {
    pub user_attributes: UserDetails,
}

impl DeleteTeam {
    pub fn new(user_attributes: UserDetails) -> Self {
        Self { user_attributes }
    }
}

impl Permission for DeleteTeam {
    type Details = TeamId;

    fn name() -> PermissionName {
        "DeleteTeam"
    }

    fn is_authorized_for(&self, team_id: <Self as Permission>::Details) -> bool {
        self.user_attributes.is_root_or_admin()
    }
}

#[cfg(test)]
mod tests {
    use crate::permission::permission::Permission;
    use crate::permission::permissions::delete_team::DeleteTeam;
    use crate::permission::user_attributes::tests::{random_user_attributes_admin, random_user_attributes_root, random_user_attributes_with};
    use uuid::Uuid;

    #[test]
    fn test_delete_team_name() {
        assert_eq!(DeleteTeam::name(), "DeleteTeam");
    }

    #[test]
    fn test_delete_team_authorization() {
        let team_id = Uuid::new_v4().into();
        let root = random_user_attributes_root(vec![]);
        let permission = DeleteTeam::new(root);
        assert!(permission.is_authorized_for(team_id));

        let admin = random_user_attributes_admin(vec![]);
        let permission = DeleteTeam::new(admin);
        assert!(permission.is_authorized_for(team_id));

        let member = random_user_attributes_with(vec![team_id.0.clone()], vec![]);
        let permission = DeleteTeam::new(member);
        assert!(!permission.is_authorized_for(team_id));

        let non_member = random_user_attributes_with(vec![], vec![]);
        let permission = DeleteTeam::new(non_member);
        assert!(!permission.is_authorized_for(team_id));
    }
}