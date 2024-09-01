use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserDetails;
use crate::user::user_id::UserId;

pub struct ReadUserDetailsPermission {
    user_attributes: UserDetails
}

impl ReadUserDetailsPermission {
    pub fn new(user_attributes: UserDetails) -> Self {
        Self { user_attributes }
    }
}

impl Permission for ReadUserDetailsPermission {
    type Details = UserId;

    fn name() -> PermissionName {
        "ReadUserDetailsPermission"
    }

    fn is_authorized_for(&self, user_id: <Self as Permission>::Details) -> bool {
        self.user_attributes.is_root_or_admin() || user_id == self.user_attributes.id
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::permission::permission::Permission;
    use crate::permission::permissions::read_user_details_permission::ReadUserDetailsPermission;
    use crate::permission::user_attributes::tests::{random_user_attributes, random_user_attributes_admin, random_user_attributes_root};

    #[test]
    fn test_read_user_details_permission_name() {
        assert_eq!(ReadUserDetailsPermission::name(), "ReadUserDetailsPermission")
    }
    
    #[test]
    fn test_read_user_details_permission_authorization() {
        let random_user = Uuid::new_v4();
        let root = random_user_attributes_root(vec![]);
        assert_ne!(root.id.0, random_user);
        let permission = ReadUserDetailsPermission::new(root);
        assert!(permission.is_authorized_for(random_user.into()));

        let random_user = Uuid::new_v4();
        let admin = random_user_attributes_admin(vec![]);
        assert_ne!(admin.id.0, random_user);
        let permission = ReadUserDetailsPermission::new(admin);
        assert!(permission.is_authorized_for(random_user.into()));

        let user = random_user_attributes();
        let random_user = user.id.0;
        assert_eq!(user.id.0, random_user);
        let permission = ReadUserDetailsPermission::new(user);
        assert!(permission.is_authorized_for(random_user.into()));
    }
    
}