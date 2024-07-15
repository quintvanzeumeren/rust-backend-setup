use crate::organisation::member::Member;

pub type PermissionName = &'static str;

pub trait Permission {

    /// Context contains the necessary information to the Permission to decide
    /// if the member is granted the permission
    type Context;

    /// Name returns the name of the permission as a 'static str
    fn name() -> PermissionName;

    /// grand checks whenever
    fn is_granted(&self, member: Member, context: <Self as Permission>::Context) -> bool;
}


// pub struct Member {
//
// }
//
// struct Resource {
//     id: Uuid,
//     resource_type: String,
//     resource_id: Uuid
// }

