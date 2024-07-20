use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use domain::permission::permission::Permission;
use domain::permission::permissions::read_organisation_users::ReadOrganisationUsers;
use domain::user::user_id::UserId;

use crate::extractors::user::user_extractor::UserExtractor;
use crate::queries::database::Database;

pub struct PermissionOf<P: Permission, RequestContent: DeserializeOwned + Into<P::Context>> {
    phantom_permission: PhantomData<P>,
    phantom_request_content: PhantomData<RequestContent>,

    pub database: Database
}

impl <P: Permission, RequestContent: DeserializeOwned + Into<P::Context>> PermissionOf<P, RequestContent> {

    pub fn create(database: Database) -> Self {
        PermissionOf {
            phantom_permission: Default::default(),
            phantom_request_content: Default::default(),
            database,
        }
    }
}

impl <RC: DeserializeOwned + Into<<ReadOrganisationUsers as Permission>::Context>> UserExtractor for PermissionOf<ReadOrganisationUsers, RC> {
    type Rejection = sqlx::Error;
    type Content = ReadOrganisationUsers;

    async fn extract(&self, user_id: UserId) -> Result<Self::Content, Self::Rejection> {
        todo!()
    }
}

