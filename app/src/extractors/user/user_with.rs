use std::sync::Arc;

use anyhow::Context;
use axum::{async_trait, Json, RequestExt};
use axum::extract::{FromRef, FromRequest, FromRequestParts, Request};
use serde::de::DeserializeOwned;

use domain::permission::permission::Permission;

use crate::app_state::AppState;
use crate::extractors::authenticated_user::authenticated_user::AuthenticatedUser;
use crate::extractors::user::permission_extractor::permission_of::PermissionOf;
use crate::extractors::user::user_extractor::UserExtractor;
use crate::handlers::internal::v1::auth::authentication_error::AuthenticationError;

struct UserWith<Extractor: UserExtractor> {
    pub content: Extractor::Content,
}

#[async_trait]
impl<P, S, D> FromRequest<S> for UserWith<PermissionOf<P, D>>
where
    S: Send + Sync,
    P: Permission,
    D: DeserializeOwned + Into<P::Context>,
    Arc<AppState>: FromRef<S>,
    PermissionOf<P, D>: UserExtractor
{
    type Rejection = AuthenticationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let mut mutable_parts = parts.clone();
        let authenticated_user = AuthenticatedUser::from_request_parts(&mut mutable_parts, state).await?;
    
        let req = Request::from_parts(parts, body);
        let body: Json<D> = Json::from_request(req, state).await.expect("todo proper error handling");
        
        // let name = P::name();
        
        let permission_extractor: PermissionOf<P, D> = PermissionOf::create(authenticated_user.state.db.clone());
        let result = permission_extractor.extract(authenticated_user.user_id)
            .await
            .context("Failed to extract permission for user")?;
    
        // let querier = Box::new(authenticated_user.state.db.clone() as dyn PermissionQuerier<P>);
        // let permission: P = querier.get_permission_for(authenticated_user.user_id).await?;
        // let has_permission = permission.is_granted_for(body.into());
        // if !has_permission { 
        //     todo!("Return proper error response")    
        // }
        
        todo!("Return proper object back to handler")
    }
}


// pub trait Permission {
//     
//     type Attributes;
// 
//     /// Name returns the str value for the permission
//     fn name() -> &'static str;
// 
//     /// grand checks whenever 
//     fn grand(&self, user_permissions: UserPermissions, attributes: <Self as Permission>::Attributes) -> bool;
// }
// 
// pub struct Member {
// 
// }
// 
// pub struct ReadOrganisationMembers;
// 
// pub struct ReadOrganisationMembersAttributes {
//     pub organisation_id: Uuid
// }
// 
// impl Permission for ReadOrganisationMembers {
//     type Attributes = ReadOrganisationMembersAttributes;
// 
//     fn name() -> &'static str {
//         "ReadOrganisationMembers"
//     }
// 
//     fn grand(&self, user_permissions: UserPermissions, attributes: <Self as Permission>::Attributes) -> bool {
//         return match user_permissions.permissions.get(ReadOrganisationMembers::name()) {
//             None => false,
//             Some(read_permissions) => {
//                 for read_permission in read_permissions {
//                     if let Some(resource) = &read_permission.resource {
//                         if resource.resource_specific_id == attributes.organisation_id { 
//                             return true
//                         }
//                     }    
//                 }
//                 
//                 false
//             }
//         }
//     }
// }
// 
// pub struct EditOwnPassword;
// 
// pub struct UserPermissions {
//     permissions: HashMap<&'static str, Vec<PermissionData>>
// }
// 
// pub struct PermissionData {
//     id: Uuid,
//     organisation_id: Uuid,
//     permission_name: String,
//     resource: Option<ResourceData>
// }
// 
// pub struct ResourceData {
//     id: Uuid,
//     organisation_id: Uuid,
//     resource_type: String,
//     resource_specific_id: Uuid
// }
// 
// 
