use crate::permission::permission::{Permission, PermissionName};

/// CreateOrganisation checks if the user can create new organisation 
pub struct CreateOrganisation;

impl Permission for CreateOrganisation {
    type Context = ();

    fn name() -> PermissionName {
        "CreateOrganisation"
    }

    fn is_granted_for(&self, _: <Self as Permission>::Context) -> bool {
        // This method can only be executed when a user was successfully created with this permission.
        // only a user that contains this permission can be created with it.
        // therefor the only possible return value is true
        true
    }
}