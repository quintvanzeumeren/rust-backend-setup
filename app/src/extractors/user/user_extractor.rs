use std::error::Error;

use domain::user::user_id::UserId;

pub trait UserExtractor {
    
    type Rejection: Error;
    type Content;
    
    async fn extract(&self, user_id: UserId) -> Result<Self::Content, Self::Rejection>;
}