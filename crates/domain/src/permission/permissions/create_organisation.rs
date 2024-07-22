use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
/// CreateOrganisation checks if the user can create new organisation
pub struct CreateOrganisation;
impl PermissionAuthorizer for CreateOrganisation {
    type ResourceInQuestion = ();

    fn name() -> PermissionName {
        "CreateOrganisation"
    }

    fn is_authorized_for(&self, _: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        // This method can only be executed when a user was successfully created with this permission.
        // only a user that contains this permission can be created with it.
        // therefor the only possible return value is true
        true
    }
}