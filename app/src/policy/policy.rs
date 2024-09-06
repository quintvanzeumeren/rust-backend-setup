use std::error::Error;
use std::sync::Arc;
use axum::async_trait;
use axum::response::IntoResponse;
use domain::user::user_id::UserId;
use crate::app_state::AppState;
use crate::policy::policy_authorization_error::PolicyRejectionError;

/// Policy represents the rules or constraints that dictate what a user can or cannot do with a
/// specific resource. It can encapsulate logic like "Can the user add a member to this team?".
/// The logic for validating whenever a user it authorized to perform the operation will in most
/// cases be contained within a specific [`domain::permission::permission`]
#[async_trait]
pub trait Policy: Sized {

    /// new is a factory method for creating a new instance of the Policy for the given user.
    async fn new(state: Arc<AppState>, user: UserId) -> Result<Self, PolicyRejectionError>;

    /// Details contains the necessary information for the Policy to understand the resource for 
    /// which the Policy is to dictate if the user is authorized or not perform an action on that 
    /// resource.
    type Details;

    /// Contract provides the actual operations that the user can perform after it was authorized
    /// by the policy. The contract can only perform the operation on the resource for which the 
    /// user was authorized for.
    type Contract;

    /// Authorize dictates if the user can or cannot do an operation.
    async fn authorize(&self, details: Self::Details) -> Result<Self::Contract, PolicyRejectionError>;

}