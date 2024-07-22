use std::collections::HashSet;

use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
use crate::permission::user_details::UserAttributes;

/// DeleteOrganisation is a permission that checks if whenever the user can delete an organisation.
pub struct DeleteOrganisation {
    pub user_attributes: UserAttributes,
    pub deletable_organisations: HashSet<OrganisationId>
}

impl PermissionAuthorizer for DeleteOrganisation {
    type ResourceInQuestion = DeleteOrganisationContext;

    fn name() -> PermissionName {
        "DeleteOrganisation"
    }

    fn is_authorized_for(&self, resource_in_question: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        // todo probably add more conditions to this delete operation
        // like cannot delete organisation of paying customers..
        let part_of_organisation = self.user_attributes.organisations.contains(&resource_in_question.organisation_to_delete);
        if part_of_organisation {
            return false
        }

        self.deletable_organisations.contains(&resource_in_question.organisation_to_delete)
    }
}

pub struct DeleteOrganisationContext {
    pub organisation_to_delete: OrganisationId
}