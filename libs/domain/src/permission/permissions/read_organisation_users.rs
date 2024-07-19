use crate::organisation::organisation_id::OrganisationId;
use crate::permission::permission::{Permission, PermissionName};
use crate::permission::resource::resource::Resource;
use crate::permission::user_details::UserDetails;

/// ReadOrganisationUsers checks if the user can read the members of an organisation
pub struct ReadOrganisationUsers {
    pub user_details: UserDetails,
    pub user_resources: Vec<Resource<OrganisationId>>
}

impl Permission for ReadOrganisationUsers {
    type Context = ReadOrganisationUsersResource;

    fn name() -> PermissionName {
        "ReadOrganisationUsers"
    }

    fn is_granted_for(&self, context: <Self as Permission>::Context) -> bool {
        let user_is_part_of_organisation = self.user_details.organisations.contains(&context.organisation_id);
        if user_is_part_of_organisation {
            return true;
        }

        let user_has_access_to_resource = self.user_resources.iter()
            .map(|r| r.resource)
            .any(|r| r == context.organisation_id);

        return user_has_access_to_resource;
    }
}

pub struct ReadOrganisationUsersResource {
    pub organisation_id: OrganisationId
}

