use password_hash::SaltString;
use secrecy::Secret;
use uuid::Uuid;
use domain::user::password::Password;
use domain::user::user_credentials::{UserCredentials};
use crate::random::_common::random_string;

pub fn random_user(password: Secret<String>, salt_string: &SaltString) -> UserCredentials {
    UserCredentials {
        id: Uuid::new_v4().into(),
        username: random_string(),
        password: Password::new(password, salt_string).expect("Failed to random new password")
    }
}