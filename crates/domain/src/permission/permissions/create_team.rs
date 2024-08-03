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
        self.user.is_root()
    }
}