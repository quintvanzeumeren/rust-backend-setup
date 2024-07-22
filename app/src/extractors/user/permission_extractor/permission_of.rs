use std::marker::PhantomData;
use axum::async_trait;
use serde::de::DeserializeOwned;

use domain::permission::permission::Permission;
use domain::permission::permissions::read_organisation_users::ReadOrganisationUsers;
use domain::user::user_id::UserId;

use crate::extractors::user::user_extractor::{UserContent, UserExtractor};
use crate::queries::database::Database;

pub struct PermissionOf<P, RequestContent>
where
    P: Permission,
    RequestContent: DeserializeOwned + Into<P::Context> + Send + Sync
{
    phantom_permission: PhantomData<P>,
    phantom_request_content: PhantomData<RequestContent>,

    database: Database
}

impl <P, RequestContent> PermissionOf<P, RequestContent>
where
    P: Permission,
    RequestContent: DeserializeOwned + Into<P::Context> + Send + Sync
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
    P: Permission,
    RC: DeserializeOwned + Into<<P as Permission>::Context> + Send + Sync
{
    type Content = P;
}

#[async_trait]
impl <RC> UserExtractor for PermissionOf<ReadOrganisationUsers, RC>
where
    RC: DeserializeOwned + Into<<ReadOrganisationUsers as Permission>::Context> + Send + Sync,
{
    type Rejection = sqlx::Error;

    async fn extract(&self, user_id: UserId) -> Result<ReadOrganisationUsers, Self::Rejection> {
        // todo implement
        Err(sqlx::Error::RowNotFound)
    }
}

