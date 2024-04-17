use chrono::{DateTime, Utc};
use secrecy::Secret;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;


// TODO: should decouple the Encryptor and Decryptor traits from the Token trades:
// Most functionality for using the token trait does not need to require either Encryptor or Decryptor. 
// Currently the UserSession API is directly coupled to using a specific token because of the need 
// to specify the Encryptor and Decryptor traits without benefit.

pub trait Token<'a, CustomClaims, EncryptionKeyType>: Encryptor<EncryptionKeyType> + Decryptor<EncryptionKeyType> + Clone
    where
        CustomClaims: Serialize + DeserializeOwned + Clone
{

    /// Returns the ID of the token.
    fn get_id(&'a self) -> &'a Uuid;

    /// Returns the subject of the token.
    fn get_subject(&'a self) -> &'a str;

    /// Returns the audience for whom the token is meant for.
    fn get_audience(&'a self) -> &'a str;

    /// Returns the issuer (creator) of the token.
    fn get_issuer(&'a self) -> &'a str;

    /// Returns when the token expires
    fn get_expiration(&'a self) -> &'a DateTime<Utc>;

    /// Returns when the token is not yet active.
    fn get_not_before(&'a self) -> &'a DateTime<Utc>;

    /// Returns when the token was issued (created).
    fn get_issued_at(&'a self) -> &'a DateTime<Utc>;

    /// Returns the additional claims that where registered on the token
    fn get_custom_claims(&'a self) -> &'a CustomClaims;
    
    /// expired returns true if the expiration time of the token was passed 
    fn expired(&'a self) -> bool {
        *self.get_expiration() < Utc::now()
    }

    /// active returns true if the time in where the token becomes active (not before) has passed
    fn active(&'a self) -> bool {
        *self.get_not_before() < Utc::now()
    }
}

pub struct EncryptedToken {
    pub token: Secret<String>,
    pub expires_at: DateTime<Utc>
}

pub trait Encryptor<EncryptionKeyType> {
    type EncryptionError;

    fn encrypt(&self, encryption_key: EncryptionKeyType) -> Result<EncryptedToken, Self::EncryptionError>;

}

pub trait Decryptor<DecryptionKeyType>: Sized {
    type DecryptionError;

    fn decrypt(encrypted_token: &Secret<String>, decryption_key: DecryptionKeyType) -> Result<Self, Self::DecryptionError>;
}