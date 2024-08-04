

pub type PermissionName = &'static str;

/// PermissionAuthorizer determines if the User has authorization to the ResourceInQuestion.
pub trait PermissionAuthorizer: Send + Sync + Sized {

    /// ResourceInQuestion contains the attributes by which the Permission identifies the resource
    /// for which it's determining authorization.
    type ResourceInQuestion;

    /// Name returns the name of the permission as a 'static str
    fn name() -> PermissionName;

    /// is_authorized determines whenever the user is authorized for the Permission.
    fn is_authorized_for(&self, resource_in_question: <Self as PermissionAuthorizer>::ResourceInQuestion) -> bool;
}