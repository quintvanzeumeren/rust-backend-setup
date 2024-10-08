use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

pub trait Token<'a>: Clone + Sized {
    type CustomClaims: Serialize + DeserializeOwned + Clone;

    fn new(
        id: Uuid,
        subject: String,
        audience: String,
        issuer: String,
        expiration: DateTime<Utc>,
        not_before: DateTime<Utc>,
        issued_at: DateTime<Utc>,
        custom_claims: Self::CustomClaims
    ) -> Self;

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
    fn get_custom_claims(&'a self) -> &'a Self::CustomClaims;
    
    /// expired returns true if the expiration time of the token was passed 
    fn expired(&'a self) -> bool;

    /// active returns true if the time in where the token becomes active (not before) has passed
    fn active(&'a self) -> bool;
}