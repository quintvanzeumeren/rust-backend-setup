use sqlx::{query_file};
use domain::user::user_id::UserId;
use crate::queries::database::Database;

impl Database {
    
    pub async fn exist_user_of(&self, user_id: UserId) -> sqlx::Result<bool> {
        let result = query_file!(
            "src/queries/exist_user_of.sql",
            user_id.0,
        ).fetch_one(self.db()).await?;
        
        Ok(result.exists.unwrap_or_else(|| false))
    }
}