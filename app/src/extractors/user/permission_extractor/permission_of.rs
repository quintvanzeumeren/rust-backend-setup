use std::marker::PhantomData;
use axum::async_trait;
use serde::de::DeserializeOwned;

use domain::permission::permission_authorizer::PermissionAuthorizer;
use domain::permission::permissions::create_team::CreateTeam;
use domain::permission::permissions::read_team_members::ReadTeamMembers;
use domain::user::user_id::UserId;

use crate::extractors::user::user_extractor::{UserContent, UserExtractor};
use crate::queries::database::Database;

pub struct PermissionOf<P, RequestContent>
where
    P: PermissionAuthorizer,
    RequestContent: DeserializeOwned + Into<P::ResourceInQuestion> + Send + Sync
{
    phantom_permission: PhantomData<P>,
    phantom_request_content: PhantomData<RequestContent>,

    database: Database
}

impl <P, RequestContent> PermissionOf<P, RequestContent>
where
    P: PermissionAuthorizer,
    RequestContent: DeserializeOwned + Into<P::ResourceInQuestion> + Send + Sync
{

    pub fn create(database: Database) -> Self {
        PermissionOf {
            phantom_permission: Default::default(),
            phantom_request_content: Default::default(),
            database,
        }
    }
}

impl<P, RC> UserContent for PermissionOf<P, RC>
where
    P: PermissionAuthorizer,
    RC: DeserializeOwned + Into<<P as PermissionAuthorizer>::ResourceInQuestion> + Send + Sync + Clone
{
    type Content = P;
    type RequestContent = RC;
}

#[async_trait]
impl <RC> UserExtractor for PermissionOf<ReadTeamMembers, RC>
where
    RC: DeserializeOwned + Into<<ReadTeamMembers as PermissionAuthorizer>::ResourceInQuestion> + Send + Sync + Clone,
{
    type Rejection = sqlx::Error;

    async fn extract(&self, user_id: &UserId) -> Result<ReadTeamMembers, Self::Rejection> {
        // todo implement
        Err(sqlx::Error::RowNotFound)
    }
}

#[async_trait]
impl<RC> UserExtractor for PermissionOf<CreateTeam, RC> 
where
    RC: DeserializeOwned + Into<<CreateTeam as PermissionAuthorizer>::ResourceInQuestion> + Send + Sync + Clone,
{
    type Rejection = sqlx::Error;

    async fn extract(&self, user_id: &UserId) -> Result<Self::Content, Self::Rejection> {
        let user_attributes = self.database.get_user_attributes(&user_id).await?;
        
        Ok(CreateTeam {
            user: user_attributes
        })
    }
}