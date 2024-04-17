use password_hash::SaltString;
use secrecy::Secret;
use uuid::Uuid;

pub fn random_string() -> String {
    Uuid::new_v4().to_string()
}

pub fn random_secret() -> Secret<String> {
    Secret::new(random_string())
}

pub fn random_salt() -> SaltString {
    SaltString::generate(&mut rand::thread_rng())
}