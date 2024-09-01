use std::fmt::{Debug, Formatter};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use lib_util::errors::errors::format_error_chain;

#[derive(thiserror::Error)]
pub enum PolicyRejectionError {
    #[error("Forbidden")]
    Forbidden,

    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

impl Debug for PolicyRejectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

impl IntoResponse for PolicyRejectionError {
    fn into_response(self) -> Response {
        match self {
            PolicyRejectionError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            PolicyRejectionError::InternalError(_) => {
                // todo log error
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}