use std::collections::HashSet;

use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission_authorizer::{PermissionAuthorizer, PermissionName};
use crate::permission::resource::resource::Resource;
use crate::permission::user_details::UserAttributes;

/// ReadOrganisationUsers checks if the user can read the members of an organisation
pub struct ReadOrganisationUsers {
    pub user_attributes: UserAttributes,
    pub resources: HashSet<Resource<OrganisationId>>
}

impl PermissionAuthorizer for ReadOrganisationUsers {
    type ResourceInQuestion = ReadOrganisationUsersResource;

    fn name() -> PermissionName {
        "ReadOrganisationUsers"
    }

    fn is_authorized_for(&self, context: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool {
        let user_is_part_of_organisation = self.user_attributes.organisations.contains(&context.organisation_id);
        if user_is_part_of_organisation {
            return true;
        }

        let user_has_access_to_resource = self.resources.iter()
            .map(|r| r.resource)
            .any(|r| r == context.organisation_id);

        return user_has_access_to_resource;
    }
}

pub struct ReadOrganisationUsersResource {
    pub organisation_id: OrganisationId
}

