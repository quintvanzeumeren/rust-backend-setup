use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission::Permission;
use crate::permission::user_details::UserDetails;
use crate::user::user_id::UserId;

#[derive(Debug, Clone)]
struct UserPermissions<P: Permission> {
    pub id: UserId,
    pub organisations: Vec<OrganisationId>,
    pub permission: P
}

impl <T: Permission> Into<UserDetails> for UserPermissions<T> {
    fn into(self) -> UserDetails {
        UserDetails {
            id: self.id,
            organisations: self.organisations,
        }
    }
}