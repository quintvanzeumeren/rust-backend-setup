use crate::organisation::organisation_id::OrganisationId;

pub struct ReadMembersOfOrganisation;

pub struct ReadMembersOfOrganisationResource {
    pub organisation_id: OrganisationId
}

// impl Permission for ReadMembersOfOrganisation {
//     type Resource = ReadMembersOfOrganisationResource;
// 
//     fn name() -> PermissionName {
//         "ReadMembersOfOrganisation"
//     }
// 
//     fn is_granted(&self, member: Member, context: <Self as Permission>::Resource) -> bool {
// 
//         if member.organisation_id == context.organisation_id {
//             return true
//         }
// 
//         return false
//     }
// }

