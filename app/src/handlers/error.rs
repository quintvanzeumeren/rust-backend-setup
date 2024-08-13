use std::fmt::{Debug, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lib_util::errors::errors::format_error_chain;
use crate::policy::policy_authorization_error::PolicyAuthorizationError;

pub type HandlerResponse<T> = Result<T, HandlerError>;

#[derive(thiserror::Error)]
pub enum HandlerError {
    
    #[error(transparent)]
    PolicyAuthorizationError(#[from] PolicyAuthorizationError),
    
    #[error(transparent)]
    InternalError(#[from] anyhow::Error)
}

impl Debug for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::PolicyAuthorizationError(e) => e.into_response(),
            HandlerError::InternalError(e) => {
                // todo log internal error prior to returning a response
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}