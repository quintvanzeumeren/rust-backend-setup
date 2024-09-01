use std::collections::HashSet;
use crate::role::role::{ROLE_ADMIN, ROLE_ROOT};
use crate::shared::slug::Slug;
use crate::team::team_id::TeamId;
use crate::user::user_id::UserId;

/// UserAttributes contains attributes of the User by which a Permission can determine if the user
/// is authorized.
#[derive(Debug, Clone)]
pub struct UserDetails {
    pub id: UserId,
    pub teams: HashSet<TeamId>,
    pub roles: HashSet<Slug>
}

impl UserDetails {
    pub fn is_root(&self) -> bool{
        self.has_role(&ROLE_ROOT.into())
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(&ROLE_ADMIN.into())
    }

    pub fn has_role(&self, role: &Slug) -> bool {
        self.roles.contains(role)
    }

    pub fn is_root_or_admin(&self) -> bool {
        self.is_root() || self.is_admin()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashSet;
    use uuid::Uuid;
    use crate::permission::user_attributes::UserDetails;
    use crate::role::role::{ROLE_ADMIN, ROLE_ROOT};
    use crate::shared::slug::Slug;

    fn user_with_roles(roles: HashSet<Slug>) -> UserDetails {
        UserDetails {
            id: Uuid::new_v4().into(),
            teams: HashSet::default(),
            roles: roles,
        }
    }

    #[test]
    fn test_has_role() {
        let user = user_with_roles(HashSet::default());
        assert!(!user.is_root());
        assert!(!user.is_admin());
        assert!(!user.is_root_or_admin());

        let mut roles = HashSet::new();
        roles.insert(ROLE_ROOT.into());
        let user = user_with_roles(roles.clone());

        assert!(user.is_root());
        assert!(!user.is_admin());
        assert!(user.is_root_or_admin());

        let mut roles = HashSet::new();
        roles.insert(ROLE_ADMIN.into());
        let user = user_with_roles(roles.clone());

        assert!(!user.is_root());
        assert!(user.is_admin());
        assert!(user.is_root_or_admin());

        roles.insert(ROLE_ROOT.into());
        let user = user_with_roles(roles.clone());
        assert!(user.is_root());
        assert!(user.is_admin());
        assert!(user.is_root_or_admin());
    }
    
    pub fn random_user_attributes() -> UserDetails {
        UserDetails {
            id: Uuid::new_v4().into(),
            teams: Default::default(),
            roles: Default::default(),
        }
    }

    pub fn random_user_attributes_with(teams: Vec<Uuid>, roles: Vec<&str>) -> UserDetails {
        UserDetails {
            id: Uuid::new_v4().into(),
            teams: teams.iter().map(|id| id.clone().into()).collect(),
            roles: roles.iter().map(|r| r.to_string().into()).collect(),
        }
    }

    pub fn random_user_attributes_root(teams: Vec<Uuid>) -> UserDetails {
        UserDetails {
            id: Uuid::new_v4().into(),
            teams: teams.iter().map(|id| id.clone().into()).collect(),
            roles: vec![Slug(ROLE_ROOT.into())].into_iter().collect(),
        }
    }

    pub fn random_user_attributes_admin(teams: Vec<Uuid>) -> UserDetails {
        UserDetails {
            id: Uuid::new_v4().into(),
            teams: teams.iter().map(|id| id.clone().into()).collect(),
            roles: vec![Slug(ROLE_ADMIN.into())].into_iter().collect(),
        }
    }
}