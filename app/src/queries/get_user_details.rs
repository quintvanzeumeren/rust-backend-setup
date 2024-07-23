use domain::permission::user_attributes::UserAttributes;
use domain::user::user_id::UserId;

use crate::queries::database::Database;

impl Database {
    #[tracing::instrument(name = "Fetching user details for user id", skip(self))]
    pub async fn get_user_details(&self, id: &UserId) -> sqlx::Result<UserAttributes> {
        todo!("Unimplemented")
    }
}
