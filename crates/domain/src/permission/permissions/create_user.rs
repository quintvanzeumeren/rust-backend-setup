use crate::permission::permission::{Permission, PermissionName};
use crate::permission::user_attributes::UserAttributes;
use crate::role::role_name::RoleName;

pub struct CreateUser {
    user_attributes: UserAttributes
}

pub struct CreateUserDetails {
    with_user_roles: Vec<String>
}

impl Permission for CreateUser {
    type Details = Vec<RoleName>;

    fn name() -> PermissionName {
        "CreateUser"
    }

    fn is_authorized_for(&self, user_roles: <Self as Permission>::Details) -> bool {
        let is_root = self.user_attributes.is_root();
        if is_root { 
            return true;
        }
        
        if self.user_attributes.is_admin() && user_roles.is_empty() { 
            return true;
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::permission::permission::Permission;
    use crate::permission::permissions::create_user::CreateUser;
    use crate::permission::user_attributes::tests::{random_user_attributes_admin, random_user_attributes_root, random_user_attributes_with};
    use crate::role::role::{ROLE_ADMIN, ROLE_ROOT};

    #[test]
    fn test_create_user_name() {
        assert_eq!(CreateUser::name(),"CreateUser")
    }
    
    #[test]
    fn test_authorized_for_create_user_for_root() {
        let permission = CreateUser {
            user_attributes: random_user_attributes_root(vec![]),
        };
        
        assert!(permission.is_authorized_for(vec![ROLE_ROOT.into()]));
        assert!(permission.is_authorized_for(vec![ROLE_ADMIN.into()]));
        assert!(permission.is_authorized_for(vec![]));
    }

    #[test]
    fn test_authorized_for_create_user_admin() {
        let permission = CreateUser {
            user_attributes: random_user_attributes_admin(vec![]),
        };

        assert!(!permission.is_authorized_for(vec![ROLE_ROOT.into()]));
        assert!(!permission.is_authorized_for(vec![ROLE_ADMIN.into()]));
        assert!(permission.is_authorized_for(vec![]));
    }

    #[test]
    fn test_authorized_for_create_user_for_user() {
        let permission = CreateUser {
            user_attributes: random_user_attributes_with(vec![], vec![]),
        };

        assert!(!permission.is_authorized_for(vec![ROLE_ROOT.into()]));
        assert!(!permission.is_authorized_for(vec![ROLE_ADMIN.into()]));
        assert!(!permission.is_authorized_for(vec![]));
    }
    
    
}