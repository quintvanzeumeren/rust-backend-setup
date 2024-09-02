

pub type PermissionName = &'static str;

/// PermissionAuthorizer determines if the User has authorization to the ResourceInQuestion.
pub trait Permission: Send + Sync + Sized {

    /// Details contains the attributes by which the Permission identifies the resource
    /// for which it's determining authorization.
    type Details;

    /// name returns the name of the permission as a 'static str
    fn name() -> PermissionName;

    /// is_authorized determines whenever the user is authorized for the Permission.
    fn is_authorized_for(&self, details: <Self as Permission>::Details) -> bool;
}