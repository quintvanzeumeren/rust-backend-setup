use domain::role::role::UserRoles;
use domain::user::user_id::UserId;
use sqlx::{query, query_file, query_file_as};

use crate::queries::database::Database;
use crate::queries::records::user_role_record::{RoleName, UserRoleRecord};

impl Database {
    
    #[tracing::instrument(
        name = "fetching user roles from db",
        skip(self),
        fields (
            user_id = %user_id,
            unknown_role_id = tracing::field::Empty,
            unknown_role = tracing::field::Empty
        )
    )]
     pub async fn get_user_roles(&self, user_id: UserId) -> sqlx::Result<UserRoles> {
        // todo for some reason this query doesn't work when using query_file!
        let user_roles = query!(
            "SELECT user_id, team_id, role AS \"role!: RoleName\" FROM user_roles WHERE user_id = $1",
            user_id.0
        ).fetch_all(self.db()).await?;

         // let mut roles = HashMap::new();
         for record in user_roles {
            match record.role {
                RoleName::Root => {}
                RoleName::Admin => {}
                RoleName::TeamManager => {}
                RoleName::Member => {}
            }
         }
         
         
         // Ok(UserRoles {
         //     user_id,
         //     roles: HashSet::from_iter(result.into_iter())
         // })
         
         todo!()
    }
}

// pub struct UserRoles {
//     pub user_id: UserId,
//     pub roles: HashSet<RoleRecord>
// }

