use std::collections::HashMap;
use anyhow::Context;
use axum::{async_trait, Json};
use axum::extract::{FromRequest, FromRequestParts, Request};
use serde::de::DeserializeOwned;
use uuid::Uuid;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;

struct AuthTest<P: Permission, D: DeserializeOwned + Into<P::Attributes>> {
    permission: P,
    data: D,
}

// #[async_trait]
// impl<S, T, D, E> FromRequestParts<S> for AuthTest<T, D, E>
// where
//     Arc<AppState>: FromRef<S>,
//     S: Send + Sync,
//     T: Permission,
//     D: DeserializeOwned + Into<T::Attributes>, 
//     E: FromRequest<D>
// {
//     type Rejection = AuthenticationError;
// 
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         
//         todo!()
//     }
// }

#[async_trait]
impl<P, S, D> FromRequest<S> for AuthTest<P,D>
where
    S: Send + Sync,
    P: Permission,
    D: DeserializeOwned + Into<P::Attributes>,
{
    type Rejection = AuthenticationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // TODO allow for more data types
        let data = Json::<D>::from_request(req, state)
            .await
            .context("Failed to json")?;

        return Err(AuthenticationError::AccessTokenHeadersInvalid)
    }
}


pub trait Permission {
    
    type Attributes;

    /// Name returns the str value for the permission
    fn name() -> &'static str;

    /// grand checks whenever 
    fn grand(&self, user_permissions: UserPermissions, attributes: <Self as Permission>::Attributes) -> bool;
}

pub struct Member {

}

pub struct ReadOrganisationMembers;

pub struct ReadOrganisationMembersAttributes {
    pub organisation_id: Uuid
}

impl Permission for ReadOrganisationMembers {
    type Attributes = ReadOrganisationMembersAttributes;

    fn name() -> &'static str {
        "ReadOrganisationMembers"
    }

    fn grand(&self, user_permissions: UserPermissions, attributes: <Self as Permission>::Attributes) -> bool {
        return match user_permissions.permissions.get(ReadOrganisationMembers::name()) {
            None => false,
            Some(read_permissions) => {
                for read_permission in read_permissions {
                    if let Some(resource) = &read_permission.resource {
                        if resource.resource_specific_id == attributes.organisation_id { 
                            return true
                        }
                    }    
                }
                
                false
            }
        }
    }
}

pub struct EditOwnPassword;

pub struct UserPermissions {
    permissions: HashMap<&'static str, Vec<PermissionData>>
}

pub struct PermissionData {
    id: Uuid,
    organisation_id: Uuid,
    permission_name: String,
    resource: Option<ResourceData>
}

pub struct ResourceData {
    id: Uuid,
    organisation_id: Uuid,
    resource_type: String,
    resource_specific_id: Uuid
}


