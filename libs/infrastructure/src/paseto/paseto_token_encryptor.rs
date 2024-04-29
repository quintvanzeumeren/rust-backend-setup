use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use chrono::DateTime;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::local;
use pasetors::token::UntrustedToken;
use pasetors::version4::V4;
use secrecy::{ExposeSecret, Secret};
use uuid::Uuid;
use lib_auth::security::encryption::decryptor::Decryptor;
use lib_auth::security::encryption::encryptor::Encryptor;

use lib_auth::security::token::token::{Token};
use lib_auth::security::token::token_encryptor::{EncryptedToken, TokenEncryptor};
use lib_util::errors::errors::format_error_chain;
use crate::paseto::paseto_claims::PasetoClaims;

#[derive(Clone)]
pub struct LocalPasetoV4TokenEncryptor {
    pub symmetrick_key: SymmetricKey<V4>,
}

impl<'a, T: Token<'a>> Encryptor<'a, T, EncryptedToken> for LocalPasetoV4TokenEncryptor {
    type EncryptionError = LocalPasetoV4EncryptionError;

    fn encrypt(&self, token: &'a T) -> Result<EncryptedToken, Self::EncryptionError> {
        let mut claims = Claims::new()?;
        claims.token_identifier(token.get_id().to_string().as_str())?;
        claims.subject(token.get_subject())?;
        claims.audience(token.get_audience())?;
        claims.issuer(token.get_issuer())?;
        claims.expiration(&token.get_expiration().to_rfc3339())?;
        claims.not_before(&token.get_not_before().to_rfc3339())?;
        claims.issued_at(&token.get_issued_at().to_rfc3339())?;

        let additional_data = serde_json::to_value(token.get_custom_claims())?;
        claims.add_additional(PasetoClaims::CUSTOM_CLAIMS, additional_data)?;

        let encrypted_token = local::encrypt(&self.symmetrick_key, &claims, None, None)
            .map_err(Self::EncryptionError::PasetoError)?;

        return Ok(EncryptedToken {
            token: Secret::new(encrypted_token),
            expires_at: token.get_expiration().clone()
        })
    }
}

#[derive(thiserror::Error)]
pub enum LocalPasetoV4EncryptionError {
    #[error(transparent)]
    PasetoError(#[from] pasetors::errors::Error),

    #[error(transparent)]
    SerializeError(#[from] serde_json::Error),
}

impl Debug for LocalPasetoV4EncryptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

impl<'a, T: Token<'a>> Decryptor<T> for LocalPasetoV4TokenEncryptor {
    type DecryptionError = LocalPasetoV4DecryptionError;

    fn decrypt(&self, encrypted_token: &Secret<String>) -> Result<T, Self::DecryptionError> {
        let local_v4_token = UntrustedToken::try_from(encrypted_token.expose_secret())?;

        // Decryption will return an error of ClaimValidation when the 'not before' > now.
        // this is duo to the ClaimValidationRules and cannot be turned off.
        // TODO write pull request to https://github.com/brycx/pasetors to fix it.
        let decrypted_token = local::decrypt(
            &self.symmetrick_key,
            &local_v4_token,
            &ClaimsValidationRules::new(),
            None,
            None
        ).map_err(|e| match e {
            pasetors::errors::Error::InvalidClaim => LocalPasetoV4DecryptionError::TokenNotYetActive,
            e => LocalPasetoV4DecryptionError::PasetoError(e),
        })?;

        let claims = decrypted_token.payload_claims()
            .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?;

        let id = get_claim(claims, PasetoClaims::TOKEN_IDENTIFIER)?;
        let id = Uuid::try_parse(id.as_str())?;

        let subject = get_claim(claims, PasetoClaims::SUBJECT)?;
        let audience = get_claim(claims, PasetoClaims::AUDIENCE)?;
        let issuer = get_claim(claims, PasetoClaims::ISSUER)?;

        let expiration = get_claim(claims, PasetoClaims::EXPIRATION)?;
        let expiration = DateTime::parse_from_rfc3339(expiration.as_str())?.to_utc();

        let not_before = get_claim(claims, PasetoClaims::NOT_BEFORE)?;
        let not_before = DateTime::parse_from_rfc3339(not_before.as_str())?.to_utc();

        let issued_at = get_claim(claims, PasetoClaims::ISSUED_AT)?;
        let issued_at = DateTime::parse_from_rfc3339(issued_at.as_str())?.to_utc();

        let custom_claims = claims.get_claim(PasetoClaims::CUSTOM_CLAIMS)
            .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?;
        let custom_claims: T::CustomClaims = serde_json::from_value(custom_claims.clone())?;

        return Ok(T::new(
            id,
            subject,
            audience,
            issuer,
            expiration,
            not_before,
            issued_at,
            custom_claims
        ))
    }
}

#[derive(thiserror::Error)]
pub enum LocalPasetoV4DecryptionError {

    #[error("Token validation failed because token `not before` > now")]
    TokenNotYetActive,

    #[error("Token was decrypted successfully but did not contain the correct amount of claims")]
    MissingClaims,

    #[error(transparent)]
    CannotParseIdentifier(#[from] uuid::Error),

    #[error(transparent)]
    CannotParse(#[from] chrono::format::ParseError),

    #[error(transparent)]
    PasetoError(#[from] pasetors::errors::Error),

    #[error(transparent)]
    SerializeError(#[from] serde_json::Error),
}

impl Debug for LocalPasetoV4DecryptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error_chain(self, f)
    }
}

fn get_claim(claims: &Claims, claim_name: &str) -> Result<String, LocalPasetoV4DecryptionError> {
    let claim = claims.get_claim(claim_name)
        .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?;

    let claim = claim.as_str()
        .ok_or(LocalPasetoV4DecryptionError::MissingClaims)?
        .to_string();

    Ok(claim)
}

impl <'a, T: Token<'a>> TokenEncryptor<'a, T> for LocalPasetoV4TokenEncryptor {}