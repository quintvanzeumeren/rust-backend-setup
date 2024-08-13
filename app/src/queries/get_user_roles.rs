use std::collections::HashSet;

use sqlx::query_file_as;

use domain::user::user_id::UserId;

use crate::queries::database::Database;
use crate::queries::records::role_record::RoleRecord;

impl Database {
    
     pub async fn get_user_roles(&self, user_id: UserId) -> sqlx::Result<UserRoles> {
        let result = query_file_as!(
            RoleRecord,
            "src/queries/get_user_roles.sql",
            user_id.0
        ).fetch_all(self.db()).await?;
         
         Ok(UserRoles {
             user_id,
             roles: HashSet::from_iter(result.into_iter())
         })
    }
}

pub struct UserRoles {
    pub user_id: UserId,
    pub roles: HashSet<RoleRecord>
}