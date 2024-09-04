use std::cmp::Ordering;
use std::collections::HashSet;
use crate::team::team_id::TeamId;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

pub type NameOfRole = &'static str;
pub const ROLE_ROOT: NameOfRole = "Root";
pub const ROLE_ADMIN: NameOfRole = "Admin";
pub const ROLE_TEAM_MANAGER: NameOfRole = "TeamManager";
pub const ROLE_MEMBER: NameOfRole = "Member";

pub type UserRoles = HashSet<Role>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Role {
    Root,
    Admin,
    TeamManager(TeamId),
    Member(TeamId)
}

impl Role {

    pub fn is_root(&self) -> bool {
        match self {
            Role::Root => true,
            _ => false
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            Role::Admin => true,
            _ => false
        }
    }

    pub fn is_team_manager_of(&self, team_id: TeamId) -> bool {
        match self {
            Role::TeamManager(ti) => ti.clone() == team_id,
            _ => false
        }
    }
    
    pub fn name(&self) -> NameOfRole {
        match self {
            Role::Root => ROLE_ROOT,
            Role::Admin => ROLE_ADMIN,
            Role::TeamManager { .. } => ROLE_TEAM_MANAGER,
            Role::Member { .. } => ROLE_MEMBER
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().to_string())
    }
}

impl PartialOrd<Self> for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> Ordering {

        match (self, other) {
            (Role::Root, Role::Root) => Ordering::Equal,
            (Role::Root, _) => Ordering::Greater,
            (_, Role::Root) => Ordering::Less,

            (Role::Admin, Role::Admin) => Ordering::Equal,
            (Role::Admin, _) => Ordering::Greater,
            (_, Role::Admin) => Ordering::Less,

            (Role::TeamManager(..), Role::TeamManager(..)) => Ordering::Equal,
            (Role::TeamManager(..), Role::Member(..)) => Ordering::Greater,

            (Role::Member(..), Role::Member(..)) => Ordering::Equal,
            (Role::Member(..), _) => Ordering::Less,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::role::role::{Role, ROLE_ADMIN, ROLE_ROOT, ROLE_TEAM_MANAGER};
    use uuid::Uuid;

    #[test]
    fn test_display() {
        let role = Role::Root;
        assert_eq!(role.to_string(), "Root");
        assert_eq!(role.to_string(), ROLE_ROOT);

        let role = Role::Admin;
        assert_eq!(role.to_string(), "Admin");
        assert_eq!(role.to_string(), ROLE_ADMIN);

        let role = Role::TeamManager(Uuid::new_v4().into());
        assert_eq!(role.to_string(), "TeamManager");
        assert_eq!(role.to_string(), ROLE_TEAM_MANAGER);
    }

    #[test]
    fn test_is_root() {
        let role = Role::Root;
        assert!(role.is_root());

        let role = Role::Admin;
        assert!(!role.is_root());

        let role = Role::TeamManager(Uuid::new_v4().into());
        assert!(!role.is_root());
    }

    #[test]
    fn test_is_admin() {
        let role = Role::Root;
        assert!(!role.is_admin());

        let role = Role::Admin;
        assert!(role.is_admin());

        let role = Role::TeamManager(Uuid::new_v4().into());
        assert!(!role.is_admin());
    }

    #[test]
    fn test_is_team_manager() {
        let team_id = Uuid::new_v4().into();
        let role = Role::Root;
        assert!(!role.is_team_manager_of(team_id));

        let role = Role::Admin;
        assert!(!role.is_team_manager_of(team_id));

        let role = Role::TeamManager(Uuid::new_v4().into());
        assert!(!role.is_team_manager_of(team_id));

        let role = Role::TeamManager(team_id);
        assert!(role.is_team_manager_of(team_id));
    }
}