use password_hash::SaltString;
use secrecy::Secret;
use uuid::Uuid;
use domain::user::password::Password;
use domain::user::user::User;

pub fn random_user(password: Secret<String>, salt_string: &SaltString) -> User {
    User {
        id: Uuid::new_v4(),
        username: Uuid::new_v4().to_string(),
        hashed_password: Password::new(password, salt_string).expect("Failed to random new password")
    }
}