use axum::async_trait;
use sqlx::Error;
use domain::permission::permissions::read_organisation_users::ReadOrganisationUsers;
use domain::user::user_id::UserId;
use crate::queries::database::Database;
use crate::queries::permissions::permission_querier::PermissionQuerier;

#[async_trait]
impl PermissionQuerier<ReadOrganisationUsers> for Database {
    async fn get_permission_for(&self, user_id: UserId) -> Result<ReadOrganisationUsers, Error> {
        todo!("implement")
    }
}