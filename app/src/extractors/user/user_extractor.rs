use std::error::Error;

use axum::async_trait;

use domain::user::user_id::UserId;

#[async_trait]
pub trait UserExtractor: Send + Sync + UserContent {

    type Rejection: Error + Send + Sync + 'static;

    async fn extract(&self, user_id: UserId) -> Result<Self::Content, Self::Rejection>;
}

pub trait UserContent {
    type Content;
    type RequestContent: Clone;
}



