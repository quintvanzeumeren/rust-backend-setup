use password_hash::SaltString;
use secrecy::Secret;
use uuid::Uuid;
use domain::user::new_user::NewUser;
use domain::user::password::Password;
use domain::user::user_credentials::{UserCredentials};
use crate::random::_common::random_string;

pub fn random_new_user(password: Secret<String>, salt_string: &SaltString) -> NewUser {
    NewUser {
        id: Uuid::new_v4().into(),
        username: random_string(),
        password: Password::new(password, salt_string).expect("Failed to random new password"),
        system_role: None,
    }
}