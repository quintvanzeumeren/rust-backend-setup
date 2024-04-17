use password_hash::Error as PasswordHashError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {

    /// IncorrectPassword means the password did not equal the value of the hash.
    #[error("Password this not match the hash")]
    PasswordInvalid,
    
    #[error("{0}")]
    Other(#[source] PasswordHashError)
}

impl From<PasswordHashError> for Error {
    fn from(error: PasswordHashError) -> Self {
        match error { 
            PasswordHashError::Password => Error::PasswordInvalid,
            _ => Error::Other(error)
        }
    }
}

// impl core::fmt::Display for Error {
//     fn fmt(
//         &self,
//         fmt: &mut core::fmt::Formatter,
//     ) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }
