use std::fmt::{Debug, Formatter};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use lib_util::errors::errors::format_error_chain;

#[derive(thiserror::Error)]
pub enum PolicyAuthorizationError {
    #[error("Forbidden")]
    Forbidden,
}

impl Debug for PolicyAuthorizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

impl IntoResponse for PolicyAuthorizationError {
    fn into_response(self) -> Response {
        match self {
            PolicyAuthorizationError::Forbidden => StatusCode::FORBIDDEN.into_response(),
        }
    }
}