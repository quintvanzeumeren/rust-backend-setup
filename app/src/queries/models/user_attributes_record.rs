use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use domain::permission::user_attributes::UserAttributes;
use domain::shared::slug::Slug;
use domain::team::team_id::TeamId;
use domain::user::user_id::UserId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserAttributesRecord {
    pub user_id: Uuid,
    pub roles: Option<Vec<String>>,
    pub teams: Option<Vec<Uuid>>
}

impl Into<UserAttributes> for UserAttributesRecord {
    fn into(self) -> UserAttributes {
        
        let teams = {
            match self.teams {
                None => HashSet::new(),
                Some(t) => t.iter().map(|id| TeamId::from(*id)).collect()
            }
        };
        
        let roles = {
            match self.roles {
                None => HashSet::new(),
                Some(t) => t.iter().map(|s| Slug::from(s.clone())).collect()
            }
        };
        
        UserAttributes {
            id: UserId(self.user_id),
            teams,
            roles
        }
    }
}