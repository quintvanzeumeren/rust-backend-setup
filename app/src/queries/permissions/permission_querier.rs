use axum::async_trait;
use domain::permission::permission::Permission;
use domain::user::user_id::UserId;

#[async_trait]
pub trait PermissionQuerier<T: Permission>: Sized {

    async fn get_permission_for(&self, user_id: UserId) -> Result<T, sqlx::Error>;
}