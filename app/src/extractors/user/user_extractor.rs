use std::error::Error;

use axum::async_trait;

use domain::user::user_id::UserId;

#[async_trait]
pub trait UserExtractor<T>: Send + Sync + UserContent {

    type Rejection: Error + Send + Sync + 'static;

    async fn extract(&self, user_id: UserId) -> Result<T, Self::Rejection>;
}

pub trait UserContent {
    type Content;
}



