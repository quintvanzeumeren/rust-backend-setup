use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
use crate::permission::user_attributes::UserAttributes;

/// CreateTeam checks if the user can create new team
pub struct CreateTeam {
    pub user: UserAttributes
}
impl PermissionAuthorizer for CreateTeam {
    type ResourceInQuestion = ();

    fn name() -> PermissionName {
        "CreateTeam"
    }

    fn is_authorized_for(&self, _: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        // This method can only be executed when a user was successfully created with this permission.
        // only a user that contains this permission can be created with it.
        // therefor the only possible return value is true
        true
    }
}