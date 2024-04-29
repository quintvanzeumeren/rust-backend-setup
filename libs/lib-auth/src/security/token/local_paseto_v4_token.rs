use std::fmt::{Debug, Formatter};

use chrono::{DateTime, Duration, Utc};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::local;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use secrecy::{ExposeSecret, Secret};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;
use lib_util::errors::errors::format_error_chain;

// use lib_util::errors::errors::format_error_chain;
// use crate::security::token::paseto_claims::PasetoClaims;
// use crate::security::token::token::{Decryptor, EncryptedToken, Encryptor, Token};
//
// #[derive(Clone, Debug)]
// pub struct LocalPasetoV4Token<CustomClaims: Serialize + DeserializeOwned> {
//     pub id: Uuid,
//     pub subject: String,
//     pub audience: String,
//     pub issuer: String,
//     pub expiration: DateTime<Utc>,
//     pub not_before: DateTime<Utc>,
//     pub issued_at: DateTime<Utc>,
//     pub custom_claims: CustomClaims,
// }
//
// impl<CustomClaims: Serialize + DeserializeOwned + Clone> LocalPasetoV4Token<CustomClaims> {
//     pub fn new(
//         subject: &str,
//         audience: &str,
//         issuer: &str,
//         valid_duration: Duration,
//         active_from: DateTime<Utc>,
//         custom_claims: CustomClaims
//     ) -> LocalPasetoV4Token<CustomClaims> {
//         LocalPasetoV4Token {
//             id: Uuid::new_v4(),
//             subject: subject.to_string(),
//             audience: audience.to_string(),
//             issuer: issuer.to_string(),
//             expiration: active_from + valid_duration,
//             not_before: active_from,
//             issued_at: Utc::now(),
//             custom_claims
//         }
//     }
// }
//
// impl<CustomClaims: Serialize + DeserializeOwned + Clone> Encryptor<&SymmetricKey<V4>> for LocalPasetoV4Token<CustomClaims> {
//     type EncryptionError = LocalPasetoV4EncryptionError;
//
//     fn encrypt(&self, encryption_key: &SymmetricKey<V4>) -> Result<EncryptedToken, Self::EncryptionError> {
//
//         let mut claims = Claims::new()?;
//         claims.token_identifier(self.get_id().to_string().as_str())?;
//         claims.subject(self.get_subject())?;
//         claims.audience(self.get_audience())?;
//         claims.issuer(self.get_issuer())?;
//         claims.expiration(&self.get_expiration().to_rfc3339())?;
//         claims.not_before(&self.get_not_before().to_rfc3339())?;
//         claims.issued_at(&self.get_issued_at().to_rfc3339())?;
//
//         let additional_data = serde_json::to_value(self.get_custom_claims())?;
//         claims.add_additional(PasetoClaims::CUSTOM_CLAIMS, additional_data)?;
//
//         let encrypted_token = local::encrypt(encryption_key, &claims, None, None)
//             .map_err(Self::EncryptionError::PasetoError)?;
//
//         return Ok(EncryptedToken {
//             token: Secret::new(encrypted_token),
//             expires_at: self.expiration.clone()
//         })
//     }
// }
//
// #[derive(thiserror::Error)]
// pub enum LocalPasetoV4EncryptionError {
//     #[error(transparent)]
//     PasetoError(#[from] pasetors::errors::Error),
//
//     #[error(transparent)]
//     SerializeError(#[from] serde_json::Error),
// }
//
// impl Debug for LocalPasetoV4EncryptionError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         format_error_chain(self, f)
//     }
// }

// impl<CustomClaims: Serialize + DeserializeOwned> Decryptor<&SymmetricKey<V4>> for LocalPasetoV4Token<CustomClaims> {
//     type DecryptionError = LocalPasetoV4DecryptionError;
//
//     fn decrypt(encrypted_token: &Secret<String>, decryption_key: &SymmetricKey<V4>) -> Result<Self, Self::DecryptionError> {
//         let local_v4_token = UntrustedToken::try_from(encrypted_token.expose_secret())?;
//
//         // Decryption will return an error of ClaimValidation when the 'not before' > now.
//         // this is duo to the ClaimValidationRules and cannot be turned off.
//         // TODO write pull request to https://github.com/brycx/pasetors to fix it.
//         let decrypted_token = local::decrypt(
//             decryption_key,
//             &local_v4_token,
//             &ClaimsValidationRules::new(),
//             None,
//             None
//         ).map_err(|e| match e {
//             pasetors::errors::Error::InvalidClaim => LocalPasetoV4DecryptionError::TokenNotYetActive,
//             e => LocalPasetoV4DecryptionError::PasetoError(e),
//         })?;
//
//         let claims = decrypted_token.payload_claims()
//             .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?;
//
//         let id = get_claim(claims, PasetoClaims::TOKEN_IDENTIFIER)?;
//         let id = Uuid::try_parse(id.as_str())?;
//
//         let subject = get_claim(claims, PasetoClaims::SUBJECT)?;
//         let audience = get_claim(claims, PasetoClaims::AUDIENCE)?;
//         let issuer = get_claim(claims, PasetoClaims::ISSUER)?;
//
//         let expiration = get_claim(claims, PasetoClaims::EXPIRATION)?;
//         let expiration = DateTime::parse_from_rfc3339(expiration.as_str())?.to_utc();
//
//         let not_before = get_claim(claims, PasetoClaims::NOT_BEFORE)?;
//         let not_before = DateTime::parse_from_rfc3339(not_before.as_str())?.to_utc();
//
//         let issued_at = get_claim(claims, PasetoClaims::ISSUED_AT)?;
//         let issued_at = DateTime::parse_from_rfc3339(issued_at.as_str())?.to_utc();
//
//         let custom_claims = claims.get_claim(PasetoClaims::CUSTOM_CLAIMS)
//             .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?;
//         let custom_claims: CustomClaims = serde_json::from_value(custom_claims.clone())?;
//
//         return Ok(Self {
//             id,
//             subject,
//             audience,
//             issuer,
//             expiration,
//             not_before,
//             issued_at,
//             custom_claims
//         })
//     }
// }
//
// fn get_claim(claims: &Claims, claim_name: &str) -> Result<String, LocalPasetoV4DecryptionError> {
//     let claim = claims.get_claim(claim_name)
//         .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?;
//
//     let claim = claim.as_str()
//         .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?
//         .to_string();
//
//     Ok(claim)
// }
//
// #[derive(thiserror::Error)]
// pub enum LocalPasetoV4DecryptionError {
//
//     #[error("Token validation failed because token `not before` > now")]
//     TokenNotYetActive,
//
//     #[error("Token was decrypted successfully but did not contain the correct amount of claims")]
//     MissingClaims,
//
//     #[error(transparent)]
//     CannotParseIdentifier(#[from] uuid::Error),
//
//     #[error(transparent)]
//     CannotParse(#[from] chrono::format::ParseError),
//
//     #[error(transparent)]
//     PasetoError(#[from] pasetors::errors::Error),
//
//     #[error(transparent)]
//     SerializeError(#[from] serde_json::Error),
// }
//
// impl Debug for LocalPasetoV4DecryptionError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         format_error_chain(self, f)
//     }
// }
//
// impl<'a, CustomClaims: Serialize + DeserializeOwned + Clone> Token<'a, CustomClaims, &SymmetricKey<V4>> for LocalPasetoV4Token<CustomClaims> {
//     fn get_id(&'a self) -> &'a Uuid {
//         &self.id
//     }
// 
//     fn get_subject(&'a self) -> &'a str {
//         self.subject.as_str()
//     }
// 
//     fn get_audience(&'a self) -> &'a str {
//         self.audience.as_str()
//     }
// 
//     fn get_issuer(&'a self) -> &'a str {
//         self.issuer.as_str()
//     }
// 
//     fn get_expiration(&'a self) -> &'a DateTime<Utc> {
//         &self.expiration
//     }
// 
//     fn get_not_before(&'a self) -> &'a DateTime<Utc> {
//         &self.not_before
//     }
// 
//     fn get_issued_at(&'a self) -> &'a DateTime<Utc> {
//         &self.issued_at
//     }
// 
//     fn get_custom_claims(&'a self) -> &'a CustomClaims {
//         &self.custom_claims
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use chrono::{DateTime, Duration, Utc};
//     use pasetors::keys::{Generate, SymmetricKey};
//     use secrecy::ExposeSecret;
//     use serde::de::DeserializeOwned;
//     use serde::Serialize;
//     use serde_json::{json, Value};
//     use uuid::Uuid;
//     use crate::security::token::local_paseto_v4_token::LocalPasetoV4Token;
//     use crate::security::token::token::{Decryptor, Encryptor, Token};
//
//     fn str() -> String {
//         Uuid::new_v4().to_string()
//     }
//
//     fn new_token<T: Serialize + DeserializeOwned + Clone>(data: T) -> LocalPasetoV4Token<T> {
//         LocalPasetoV4Token::new(
//             str().as_str(),
//             str().as_str(),
//             str().as_str(),
//             Duration::hours(1),
//             Utc::now() - Duration::minutes(15),
//             data
//         )
//     }
//
//     fn data_json() -> Value {
//         json!({
//             "user_id": Uuid::new_v4(),
//             "firstname": str(),
//             "lastname": str(),
//         })
//     }
//
//     fn is_within_second(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> bool {
//         let difference = (dt1 - dt2).num_seconds().abs();
//         difference <= 1
//     }
//
//     #[test]
//     fn test_new_token_creation() {
//         let subject = str();
//         let audience = str();
//         let issuer = str();
//         let active_duration = Duration::hours(2);
//         let active_from = Utc::now() + Duration::minutes(12);
//         let data = data_json();
//
//         let issued_at = Utc::now();
//         let token = LocalPasetoV4Token::new(
//             subject.as_str(),
//             audience.as_str(),
//             issuer.as_str(),
//             active_duration.clone(),
//             active_from.clone(),
//             data.clone()
//         );
//
//         assert_eq!(token.id.get_version_num(), 4);
//         assert_eq!(token.id, *token.get_id());
//
//         assert_eq!(token.subject, subject);
//         assert_eq!(token.get_subject(), subject);
//
//         assert_eq!(token.audience, audience);
//         assert_eq!(token.get_audience(), audience);
//
//         assert_eq!(token.issuer, issuer);
//         assert_eq!(token.get_issuer(), issuer);
//
//         let expiration = active_from + active_duration;
//         assert_eq!(token.expiration, expiration);
//         assert_eq!(*token.get_expiration(), expiration);
//
//         assert_eq!(token.not_before, active_from);
//         assert_eq!(*token.get_not_before(), active_from);
//
//         assert!(is_within_second(token.issued_at, issued_at));
//         assert!(is_within_second(*token.get_issued_at(), issued_at));
//
//         assert_eq!(token.custom_claims, data);
//         assert_eq!(*token.get_custom_claims(), data)
//     }
//
//     #[test]
//     fn test_encryption_and_decryption() {
//         let encryption_key = SymmetricKey::generate().expect("Failed to random key");
//         let token = new_token(data_json());
//
//         let encrypted_token = token.encrypt(&encryption_key)
//             .expect("Failed to encrypt token");
//
//         let decrypted_token: LocalPasetoV4Token<Value> = LocalPasetoV4Token::decrypt(&encrypted_token.token, &encryption_key)
//             .expect("Failed to decrypt valid encryption of PasetoV4Token");
//
//         assert_eq!(token.id, decrypted_token.id);
//         assert_eq!(token.subject, decrypted_token.subject);
//         assert_eq!(token.audience, decrypted_token.audience);
//         assert_eq!(token.issuer, decrypted_token.issuer);
//         assert_eq!(token.expiration, decrypted_token.expiration);
//         assert_eq!(token.not_before, decrypted_token.not_before);
//         assert_eq!(token.issued_at, decrypted_token.issued_at);
//         assert_eq!(token.issued_at, decrypted_token.issued_at);
//         assert_eq!(token.custom_claims, decrypted_token.custom_claims);
//     }
//
//     #[test]
//     fn test_encryption_to_create_unique_tokens_every_time() {
//         let encryption_key = &SymmetricKey::generate().expect("Failed to random key");
//         let token = new_token(data_json());
//
//         let initial_encryption = token.encrypt(encryption_key)
//             .expect("Failed to encrypt token");
//         for _ in (1..10).rev() {
//             let other = token.encrypt(encryption_key)
//                 .expect("Failed to encrypt token");
//             assert_ne!(*initial_encryption.token.expose_secret(), *other.token.expose_secret())
//         }
//     }
// }
