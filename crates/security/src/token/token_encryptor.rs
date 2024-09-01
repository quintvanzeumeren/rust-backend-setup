use chrono::{DateTime, Utc};
use secrecy::Secret;
use crate::encryption::decryptor::Decryptor;
use crate::encryption::encryptor::Encryptor;
use crate::token::token::{Token};

pub struct EncryptedToken {
    pub token: Secret<String>,
    pub expires_at: DateTime<Utc>
}

pub trait TokenEncryptor<'a, T: Token<'a>>: Encryptor<'a, T, EncryptedToken> + Decryptor<T> {}