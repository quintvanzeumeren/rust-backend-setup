use crate::organisation::organisation_id::OrganisationId;
use crate::user::user_id::UserId;

pub struct UserDetails {
    pub id: UserId,
    pub organisations: Vec<OrganisationId>
}