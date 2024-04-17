use argon2::ARGON2ID_IDENT;
use enum_dispatch::enum_dispatch;
use password_hash::{Ident, PasswordHash, SaltString};
use secrecy::Secret;
use crate::security::hash::argon2::Argon2Scheme;
use crate::security::hash::error::Error;
use crate::security::hash::schema::SchemaDispatch::Argon2ID;


/// DEFAULT_SCHEME is the hash scheme all passwords should be using
pub const DEFAULT_SCHEME: Ident = ARGON2ID_IDENT;

/// Scheme is defined common implementation for the hashing and validation of passwords
/// The DEFAULT_SCHEME represents the latest scheme each hash should use.
#[enum_dispatch]
pub trait Scheme {
    fn hash<'a>(&self, to_hash: Secret<String>, salt_string: &'a SaltString) -> password_hash::Result<PasswordHash<'a>>;

    fn validate(&self, password: &Secret<String>, password_hash: &PasswordHash) -> Result<(), Error>;
}

#[enum_dispatch(Scheme)]
pub enum SchemaDispatch {
    Argon2ID(Argon2Scheme)
}

/// get_scheme returns the scheme which represents equals the Ident
pub fn get_scheme(scheme_name: &Ident) -> Option<SchemaDispatch> {
    match *scheme_name {
        ARGON2ID_IDENT => Some(Argon2ID(Argon2Scheme)),
        _ => None
    }
}

/// is_latest_schema checks if the schema of the hash uses the latest DEFAULT_SCHEME
pub fn is_latest_schema(schema_of_hash: &Ident) -> bool {
    return *schema_of_hash == DEFAULT_SCHEME
}

/// get_latest_schema returns the current DEFAULT SCHEMA
pub fn get_latest_scheme() -> impl Scheme {
    return get_scheme(&DEFAULT_SCHEME).unwrap()
}

#[cfg(test)]
mod tests {
    use argon2::ARGON2ID_IDENT;
    use password_hash::Ident;
    use crate::security::hash::schema::{DEFAULT_SCHEME, get_latest_scheme, get_scheme, is_latest_schema};

    #[test]
    fn test_use_right_default_scheme() {
        assert_eq!(DEFAULT_SCHEME, ARGON2ID_IDENT);
        assert!(is_latest_schema(&DEFAULT_SCHEME))
    }

    #[test]
    fn test_has_latest_schema() {
        // no panic
        let _ = get_latest_scheme();

        assert!(get_scheme(&DEFAULT_SCHEME).is_some());
    }

    #[test]
    fn test_schema_retrieval() {
        assert!(get_scheme(&DEFAULT_SCHEME).is_some());
    }

    #[test]
    fn test_retrieving_none_existing_schema() {
        let ident = Ident::new("mysuperalgro").unwrap();
        let scheme = get_scheme(&ident);
        assert!(scheme.is_none());
    }
}

