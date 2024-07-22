use axum::async_trait;
use domain::permission::permission_authorizer::PermissionAuthorizer;
use domain::user::user_id::UserId;

#[async_trait]
pub trait PermissionQuerier<T: PermissionAuthorizer>: Sized {

    async fn get_permission_for(&self, user_id: UserId) -> Result<T, sqlx::Error>;
}