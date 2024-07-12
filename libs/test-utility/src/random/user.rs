use password_hash::SaltString;
use secrecy::Secret;
use uuid::Uuid;
use domain::user::password::Password;
use domain::user::user::{Admin, EndUser, User};
use crate::random::_common::random_string;

pub fn random_user(password: Secret<String>, salt_string: &SaltString) -> User<EndUser> {
    User::new(
        Uuid::new_v4().into(),
        random_string(),
        Password::new(password, salt_string).expect("Failed to random new password")
    )
}

pub fn random_admin(password: Secret<String>, salt_string: &SaltString) -> User<Admin> {
    User::new(
        Uuid::new_v4().into(),
        random_string(),
        Password::new(password, salt_string).expect("Failed to random new password")
    )
}