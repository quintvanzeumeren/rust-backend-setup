use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type NameOfRole = &'static str;
pub const ROLE_ROOT: NameOfRole = "Root";
pub const ROLE_ADMIN: NameOfRole = "Admin";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SystemRole {
    Root,
    Admin,
}

impl SystemRole {

    pub fn is_root(&self) -> bool {
        *self == SystemRole::Root
    }

    pub fn is_admin(&self) -> bool {
        *self == SystemRole::Admin
    }
    
    pub fn name(&self) -> NameOfRole {
        match self {
            SystemRole::Root => ROLE_ROOT,
            SystemRole::Admin => ROLE_ADMIN,
        }
    }
}

impl Display for SystemRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::role::role::{SystemRole, ROLE_ADMIN, ROLE_ROOT, ROLE_TEAM_MANAGER};

    #[test]
    fn test_display() {
        let role = SystemRole::Root;
        assert_eq!(role.to_string(), "Root");
        assert_eq!(role.to_string(), ROLE_ROOT);

        let role = SystemRole::Admin;
        assert_eq!(role.to_string(), "Admin");
        assert_eq!(role.to_string(), ROLE_ADMIN);
    }

    #[test]
    fn test_is_root() {
        let role = SystemRole::Root;
        assert!(role.is_root());

        let role = SystemRole::Admin;
        assert!(!role.is_root());
    }

    #[test]
    fn test_is_admin() {
        let role = SystemRole::Root;
        assert!(!role.is_admin());

        let role = SystemRole::Admin;
        assert!(role.is_admin());
    }
}