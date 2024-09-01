use std::error::Error;
use std::sync::Arc;
use axum::async_trait;
use axum::response::IntoResponse;
use domain::user::user_id::UserId;
use crate::app_state::AppState;

/// Policy represents the rules or constraints that dictate what a user can or cannot do with a
/// specific resource. It can encapsulate logic like "Can the user add a member to this team?".
/// The logic for validating whenever a user it authorized to perform the operation will in most
/// cases be contained within a specific [`domain::permission::permission`]
#[async_trait]
pub trait Policy: Sized {

    /// Rejection to be returned when the policy couldn't be created.
    type Rejection: Error + Send + Sync + 'static;

    /// new is a factory method for creating a new instance of the Policy for the user.
    async fn new(state: Arc<AppState>, user_in_question: UserId) -> Result<Self, Self::Rejection>;

    /// Details contains information for the Policy to understand the resource in question for which
    /// Policy is to dictate if the user is authorized or not.
    type Details;

    /// Contract provides the actual operations that the user can perform after it was authorized
    /// by the policy. The contract can only perform the operation that the user was authorized for.
    type Contract;

    /// AuthenticationRejection is returned when the user either does not have authorization to
    /// perform the given operation, or something went wrong when determining
    /// if the user is authorized.
    type AuthorizationRejection: IntoResponse + Error;

    /// Authorize dictates if the user can or cannot do an operation.
    fn authorize(&self, details: Self::Details) -> Result<Self::Contract, Self::AuthorizationRejection>;

}