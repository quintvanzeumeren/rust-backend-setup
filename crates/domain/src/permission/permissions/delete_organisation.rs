use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission::{Permission, PermissionName};
use crate::permission::resource::resource::Resource;


/// DeleteOrganisation is a permission that checks if whenever the user can delete an organisation.
pub struct DeleteOrganisation {
    resources: Vec<Resource<OrganisationId>>
}

impl Permission for DeleteOrganisation {
    type Context = DeleteOrganisationContext;

    fn name() -> PermissionName {
        "DeleteOrganisation"
    }

    fn is_authorized(&self, context: <Self as Permission>::Context) -> bool {
        self.resources.iter()
            .map(|r| r.resource)
            .any(|r| r == context.organisation_to_delete)
    }
}

pub struct DeleteOrganisationContext {
    pub organisation_to_delete: OrganisationId
}