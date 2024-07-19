use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission::{Permission, PermissionName};
use crate::permission::resource::resource::Resource;

/// AddUserToOrganisation checks if a user can add another user to an organisation
pub struct AddUserToOrganisation {
    pub resources: Vec<Resource<OrganisationId>>
}

impl Permission for AddUserToOrganisation {
    type Context = AddUserToOrganisationContext;

    fn name() -> PermissionName {
        "AddUserToOrganisation"
    }

    fn is_granted_for(&self, context: <Self as Permission>::Context) -> bool {
        self.resources.iter()
            .map(|r| r.resource)
            .any(|r| r == context.organisation_id)
    }
}

pub struct AddUserToOrganisationContext {
    organisation_id: OrganisationId
}