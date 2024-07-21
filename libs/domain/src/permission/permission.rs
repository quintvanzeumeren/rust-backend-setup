

pub type PermissionName = &'static str;

pub trait Permission: Send + Sync {

    /// Context contains the necessary information to the Permission to decide
    /// if the member is granted the permission
    type Context;

    /// Name returns the name of the permission as a 'static str
    fn name() -> PermissionName;

    /// is_granted_for validates whenever the user has permission to do something with the resource.
    fn is_granted_for(&self, context: <Self as Permission>::Context) -> bool;
}