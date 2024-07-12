use sqlx::query_file;
use domain::user::user_id::UserId;
use crate::queries::database::Database;

impl Database {
    pub(crate) async fn is_admin(&self, user_id: &UserId) -> Result<bool, sqlx::Error> {

        let result = query_file!("src/queries/is_admin.sql", user_id.0)
            .fetch_optional(self.db())
            .await?;

        return match result {
            None => Ok(false),
            Some(record) => {
                Ok(record.admin)
            }
        }
    }
}
