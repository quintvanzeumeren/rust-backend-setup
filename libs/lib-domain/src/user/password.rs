use std::fmt::{Debug, Formatter};
use password_hash::{PasswordHash, SaltString};
use secrecy::{ExposeSecret, Secret};
use lib_auth::security::hash::error::Error;
use lib_auth::security::hash::schema::{get_latest_scheme, get_scheme, is_latest_schema, Scheme};
use lib_util::errors::errors::format_error_chain;

pub struct Password {
    hash: Secret<String>
}

impl Password {

    pub fn new(password: Secret<String>, salt: &SaltString) -> password_hash::Result<Self> {
        let latest_scheme = get_latest_scheme();
        let hash = latest_scheme.hash(password, salt)?;
        return Ok(Password {
            hash: Secret::new(hash.to_string())
        })
    }

    pub fn hash_string(&self) -> &Secret<String> {
        &self.hash
    }

    /// matches verifies is the password_to_match matches with the hash of the password
    /// Note: this is an expensive operation i.e. can block the event loop.
    pub fn matches(&self, submitted_password: &Secret<String>) -> Result<MatchResult, MatchError> {
        let password_hash = self.get_password_hash();

        let password_algorithm = password_hash.algorithm;
        let scheme = get_scheme(&password_algorithm)
            .ok_or(MatchError::NoHashSchemeForPassword(password_algorithm.to_string()))?;

        return match scheme.validate(&submitted_password, &password_hash) {
            Ok(_) => match is_latest_schema(&password_hash.algorithm) {
                true => Ok(MatchResult::Matches),
                false => Ok(MatchResult::MatchesButSchemeOutdated),
            },
            Err(err) => match err {
                Error::PasswordInvalid => Ok(MatchResult::DoesNotMatch),
                Error::Other(e) => Err(MatchError::ErrorWhileMatching(e))
            }
        }
    }

    fn get_password_hash(&self) -> PasswordHash {
        PasswordHash::new(self.hash.expose_secret().as_str()).unwrap()
    }
}

pub enum NewPasswordError {
    /// NoHashSchemeForPassword is returned when there is no proper
    NoHashSchemeForPassword
}

#[derive(thiserror::Error, PartialEq)]
pub enum MatchError {
    /// NoHashSchemeForPassword is returned the is no scheme that matches the
    /// algorithm of the password and therefor cannot validate it.
    #[error("No scheme was found for hash with algorithm of: {0}")]
    NoHashSchemeForPassword(String),

    /// ErrorWhileMatching is not
    #[error("Error while trying to match password with the hash")]
    ErrorWhileMatching(#[source] password_hash::Error)
}

impl Debug for MatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

#[derive(Debug, PartialEq)]
pub enum MatchResult {
    /// DoesNotMatch is returned when the submitted password did not match the password hash
    DoesNotMatch,

    /// Matches is returned when the submitted password is the password
    Matches,

    /// MatchesButSchemeOutdated is returned when the password matches
    /// but the password hash needs to be updated to a newer password Scheme
    MatchesButSchemeOutdated
}


impl TryFrom<&str> for Password {
    type Error = password_hash::Error;

    fn try_from(hash: &str) -> Result<Self, password_hash::Error> {
        Password::try_from(hash.to_string())
    }
}

impl TryFrom<PasswordHash<'_>> for Password {
    type Error = password_hash::Error;
    fn try_from(hashed_password: PasswordHash) -> Result<Self, Self::Error> {
        Password::try_from(hashed_password.to_string())
    }
}

impl TryFrom<String> for Password {
    type Error = password_hash::Error;

    fn try_from(hash: String) -> Result<Self, Self::Error> {
        PasswordHash::new(&hash)?;
        return Ok(Password {
            hash: Secret::new(hash)
        })
    }
}

#[cfg(test)]
mod tests {
    use password_hash::{Ident, PasswordHash, SaltString};
    use secrecy::{ExposeSecret, Secret};
    use uuid::Uuid;
    use crate::user::password::{MatchError, MatchResult, Password};

    fn get_salt() -> SaltString {
        SaltString::generate(&mut rand::thread_rng())
    }

    fn password(salt_string: &SaltString) -> (Secret<String>, Password) {
        let pw1 = Secret::new(Uuid::new_v4().to_string());
        let password1 = Password::new(pw1.clone(), &salt_string).unwrap();

        return (pw1, password1)
    }

    #[test]
    fn password_verifies_password() {
        let salt = get_salt();
        let (pw1, password1) = password(&salt);

        assert_eq!(password1.matches(&pw1).unwrap(), MatchResult::Matches)
    }

    #[test]
    fn password_rejects_invalid_password() {
        let salt = get_salt();
        let (pw1, password1) = password(&salt);
        let pw2 = Secret::new(Uuid::new_v4().to_string());

        // Correct password
        assert_eq!(password1.matches(&pw1).unwrap(), MatchResult::Matches);

        // Incorrect password
        assert_eq!(password1.matches(&pw2).unwrap(), MatchResult::DoesNotMatch);
    }

    #[test]
    fn create_password_from() {
        let salt = get_salt();
        let (pw1, password1) = password(&salt);

        let hash_str = password1.hash_string().expose_secret();
        let result = Password::try_from(hash_str.as_str());

        // Check if parsing went ok
        assert!(result.is_ok());
        assert_eq!(result.unwrap().hash_string().expose_secret(), password1.hash_string().expose_secret());
    }

    #[test]
    fn none_existing_algorithm_should_return_no_hash_scheme_for_password() {
        let salt = get_salt();
        let (pw1, password1) = password(&salt);

        let pw = PasswordHash::new(password1.hash_string().expose_secret())
            .expect("Failed to create password hash from password");
        
        let fake_hash = PasswordHash {
            algorithm: Ident::new("something").unwrap(),
            version: pw.version,
            params: pw.params,
            salt: pw.salt,
            hash: pw.hash,
        };
    
        let result = Password::try_from(fake_hash.clone());
        assert!(result.is_ok());
    
        let fakepw = result.unwrap();
        assert_eq!(
            fakepw.matches(&pw1).err().unwrap(),
            MatchError::NoHashSchemeForPassword("something".to_string())
        )
    }
    
    #[test]
    fn test_to_string() {
        let salt = get_salt();
        let (_, password1) = password(&salt);
        assert_eq!(
            password1.hash_string().expose_secret().clone(),
            password1.hash.expose_secret().clone()
        );
    }

}