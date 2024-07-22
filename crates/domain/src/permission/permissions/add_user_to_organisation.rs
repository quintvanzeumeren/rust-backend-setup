use std::collections::HashSet;

use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};

/// AddUserToOrganisation checks if a user can add another user to an organisation
pub struct AddUserToOrganisation {
    pub organisations_where_users_can_be_added_to: HashSet<OrganisationId>
}

impl PermissionAuthorizer for AddUserToOrganisation {
    type ResourceInQuestion = AddUserToOrganisationContext;

    fn name() -> PermissionName {
        "AddUserToOrganisation"
    }

    fn is_authorized_for(&self, context: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        self.organisations_where_users_can_be_added_to.contains(&context.organisation_to_gain_user)
    }
}

pub struct AddUserToOrganisationContext {
    organisation_to_gain_user: OrganisationId
}