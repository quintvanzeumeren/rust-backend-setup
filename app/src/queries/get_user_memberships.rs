use crate::queries::database::Database;
use domain::team::membership::Membership;
use domain::user::user_id::UserId;
use sqlx::query_file_as;
use std::collections::HashSet;

impl Database {
    
     pub async fn get_user_memberships(&self, user_id: UserId) -> sqlx::Result<HashSet<Membership>> {
        let memberships = query_file_as!(
            Membership,
            "src/queries/get_user_memberships.sql",
            user_id.0,
        ).fetch_all(self.db()).await?;

        Ok(HashSet::from_iter(memberships))
    }
}