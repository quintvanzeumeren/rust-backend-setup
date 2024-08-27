use domain::permission::user_attributes::UserAttributes;
use domain::role::role::{ROLE_ADMIN, ROLE_ROOT};
use domain::shared::slug::Slug;
use uuid::Uuid;

pub fn random_user_attributes() -> UserAttributes {
    UserAttributes {
        id: Uuid::new_v4().into(),
        teams: Default::default(),
        roles: Default::default(),
    }
}

pub fn random_user_attributes_with(teams: &[Uuid], roles: &[&str]) -> UserAttributes {
    UserAttributes {
        id: Uuid::new_v4().into(),
        teams: teams.iter().map(|id| id.clone().into()).collect(),
        roles: roles.iter().map(|r| r.to_string().into()).collect(),
    }
}

pub fn random_user_attributes_root(teams: &[Uuid]) -> UserAttributes {
    UserAttributes {
        id: Uuid::new_v4().into(),
        teams: teams.iter().map(|id| id.clone().into()).collect(),
        roles: vec![Slug(ROLE_ROOT.into())].into_iter().collect(),
    }
}

pub fn random_user_attributes_admin(teams: &[Uuid]) -> UserAttributes {
    UserAttributes {
        id: Uuid::new_v4().into(),
        teams: teams.iter().map(|id| id.clone().into()).collect(),
        roles: vec![Slug(ROLE_ADMIN.into())].into_iter().collect(),
    }
}
