use std::fmt::{Display, Formatter};

use uuid::Uuid;
use crate::abac::resource::resource_type::{ResourceName, ResourceType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrganisationId(pub Uuid);

impl From<Uuid> for OrganisationId {
    fn from(value: Uuid) -> Self {
        OrganisationId(value)
    }
}

impl Display for OrganisationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ResourceType for OrganisationId {
    fn name() -> ResourceName {
        "Organisation"
    }
}

