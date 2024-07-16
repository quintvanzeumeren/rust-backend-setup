mod read_members_of_organisation;

use enum_dispatch::enum_dispatch;
use crate::abac::permissions::read_members_of_organisation::ReadMembersOfOrganisation;

#[enum_dispatch(Scheme)]
pub enum Permissions {
    ReadMembersOfOrganisation(ReadMembersOfOrganisation)
}

