use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserDetails;

/// CreateTeam checks if the user can create new team
pub struct CreateTeam {
    pub user: UserDetails
}

impl CreateTeam {
    pub fn new(user: UserDetails) -> Self {
        Self { user }
    }
}

impl Permission for CreateTeam {
    type Details = ();

    fn name() -> PermissionName {
        "CreateTeam"
    }

    fn is_authorized_for(&self, _: <Self as Permission>::Details) -> bool {
        self.user.is_root_or_admin()
    }
}

#[cfg(test)]
mod tests {
    use crate::permission::permission::Permission;
    use crate::permission::permissions::create_team::CreateTeam;
    use crate::permission::user_attributes::tests::{random_user_attributes_admin, random_user_attributes_root, random_user_attributes_with};

    #[test]
    fn test_create_team_name() {
        assert_eq!(CreateTeam::name(), "CreateTeam");
    }

    #[test]
    fn test_create_team_authorization() {
        let root = random_user_attributes_root(vec![]);
        let permission = CreateTeam::new(root);
        assert!(permission.is_authorized_for(()));

        let admin = random_user_attributes_admin(vec![]);
        let permission = CreateTeam::new(admin);
        assert!(permission.is_authorized_for(()));

        let user = random_user_attributes_with(vec![], vec![]);
        let permission = CreateTeam::new(user);
        assert!(!permission.is_authorized_for(()));
    }
}