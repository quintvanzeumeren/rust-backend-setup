use std::fmt::{Debug, Formatter};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use infrastructure::paseto::paseto_token_encryptor::LocalPasetoV4DecryptionError;
use lib_util::errors::errors::format_error_chain;

use crate::util::handlers::InternalErrorResponse;

pub type AuthenticationResult<T> = Result<T, AuthenticationError>;

#[derive(thiserror::Error)]
pub enum AuthenticationError {
    #[error("Request headers for the access token is invalid")]
    AccessTokenHeadersInvalid,

    #[error("Received credentials are incorrect")]
    CredentialsInvalid,

    #[error("Received token was invalid")]
    TokenInvalid,

    #[error("Session for the token is not active")]
    SessionNotActive,

    #[error(transparent)]
    TokenDecryptionError(#[from] LocalPasetoV4DecryptionError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        match self {
            AuthenticationError::AccessTokenHeadersInvalid
            | AuthenticationError::SessionNotActive
            | AuthenticationError::CredentialsInvalid
            | AuthenticationError::TokenInvalid => StatusCode::UNAUTHORIZED.into_response(),
            AuthenticationError::TokenDecryptionError(e) => match e {
                LocalPasetoV4DecryptionError::TokenNotYetActive => {
                    StatusCode::UNAUTHORIZED.into_response()
                }
                _ => InternalErrorResponse::from(tracing::Span::current()).into_response(),
            },
            AuthenticationError::UnexpectedError(_) => {
                InternalErrorResponse::from(tracing::Span::current()).into_response()
            }
        }
    }
}
