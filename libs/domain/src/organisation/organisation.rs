use crate::abac::resource::resource_type::{ResourceName, ResourceType};
use crate::organisation::organisation_id::OrganisationId;
use crate::shared::slug::Slug;

#[derive(Clone, Debug, PartialEq)]
pub struct Organisation {
    id: OrganisationId,
    name: String,
    slug: Slug
}