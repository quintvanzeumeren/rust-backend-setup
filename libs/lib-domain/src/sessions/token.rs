use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use lib_auth::security::token::token::Token;

#[derive(Clone, Debug)]
pub struct UserSessionToken<CC: Serialize + DeserializeOwned + Clone> {
    pub id: Uuid,
    pub subject: String,
    pub audience: String,
    pub issuer: String,
    pub expiration: DateTime<Utc>,
    pub not_before: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub custom_claims: CC
}

impl <'a, CC: Serialize + DeserializeOwned + Clone> Token<'a> for UserSessionToken<CC> {
    type CustomClaims = CC;

    fn new(
        id: Uuid,
        subject: String,
        audience: String,
        issuer: String,
        expiration: DateTime<Utc>,
        not_before: DateTime<Utc>,
        issued_at: DateTime<Utc>,
        custom_claims: Self::CustomClaims
    ) -> Self {
        Self {
            id,
            subject,
            audience,
            issuer,
            expiration,
            not_before,
            issued_at,
            custom_claims,
        }
    }

    fn get_id(&'a self) -> &'a Uuid {
        &self.id
    }

    fn get_subject(&'a self) -> &'a str {
        self.subject.as_str()
    }

    fn get_audience(&'a self) -> &'a str {
        self.audience.as_str()
    }

    fn get_issuer(&'a self) -> &'a str {
        self.issuer.as_str()
    }

    fn get_expiration(&'a self) -> &'a DateTime<Utc> {
        &self.expiration
    }

    fn get_not_before(&'a self) -> &'a DateTime<Utc> {
        &self.not_before
    }

    fn get_issued_at(&'a self) -> &'a DateTime<Utc> {
        &self.issued_at
    }

    fn get_custom_claims(&'a self) -> &'a Self::CustomClaims {
        &self.custom_claims
    }
}