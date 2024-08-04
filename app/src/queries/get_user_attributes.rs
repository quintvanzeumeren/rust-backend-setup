use sqlx::{Error, query_file, query_file_as};
use domain::permission::user_attributes::UserAttributes;
use domain::user::user_id::UserId;

use crate::queries::database::Database;
use crate::queries::models::user_attributes_record::UserAttributesRecord;

impl Database {
    #[tracing::instrument(name = "Fetching user details for user id", skip(self))]
    pub async fn get_user_attributes(&self, id: &UserId) -> sqlx::Result<UserAttributes> {
        let result = query_file!(
            "src/queries/get_user_attributes.sql",
            id.0,
        ).fetch_one(self.db()).await;
        
        // todo seperate query into multiple smaller ones, await then at the same time.
        
        return match result {
            Ok(record) => {
                let r = UserAttributesRecord {
                    user_id: record.user_id,
                    roles: record.roles,
                    teams: record.teams,
                };
                
                Ok(r.into())
            },
            Err(e) => {
                println!("${e}");
                Err(e)
            }
        }
        
        // user_attributes_record
        // 
        //         Ok(user_attributes_record.into())
    }
}
