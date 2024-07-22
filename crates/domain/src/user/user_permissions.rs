use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission_authorizer::PermissionAuthorizer;
use crate::permission::user_details::UserAttributes;
use crate::user::user_id::UserId;

#[derive(Debug, Clone)]
struct UserPermissions<P: PermissionAuthorizer> {
    pub id: UserId,
    pub organisations: Vec<OrganisationId>,
    pub permission: P
}

impl <T: PermissionAuthorizer> Into<UserAttributes> for UserPermissions<T> {
    fn into(self) -> UserAttributes {
        UserAttributes {
            id: self.id,
            organisations: self.organisations,
        }
    }
}