use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use lib_auth::security::token::token::Token;
use crate::shared::activation_time::ActivationTime;
use crate::shared::expiration::Expiration;

#[derive(Clone, Debug)]
pub struct UserSessionToken<CC: Serialize + DeserializeOwned + Clone> {
    pub id: Uuid,
    pub subject: String,
    pub audience: String,
    pub issuer: String,
    pub expiration: Expiration,
    pub not_before: ActivationTime,
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
            expiration: expiration.into(),
            not_before: not_before.into(),
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
        &self.expiration.0
    }

    fn get_not_before(&'a self) -> &'a DateTime<Utc> {
        &self.not_before.0.0
    }

    fn get_issued_at(&'a self) -> &'a DateTime<Utc> {
        &self.issued_at
    }

    fn get_custom_claims(&'a self) -> &'a Self::CustomClaims {
        &self.custom_claims
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use serde_json::{json, Value};
    use uuid::Uuid;
    use lib_auth::security::token::token::Token;
    use crate::sessions::user_session_token::UserSessionToken;

    fn str() -> String {
        Uuid::new_v4().to_string()
    }

    fn new_token<T: Serialize + DeserializeOwned + Clone>(data: T) -> UserSessionToken<T> {
        let now = Utc::now();
        UserSessionToken::new(
            Uuid::new_v4(),
            str(),
            str(),
            str(),
            now - Duration::hours(1),
            now - Duration::minutes(15),
            now,
            data
        )
    }

    fn data_json() -> Value {
        json!({
            "user_id": Uuid::new_v4(),
            "firstname": str(),
            "lastname": str(),
        })
    }

    fn is_within_second(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> bool {
        let difference = (dt1 - dt2).num_seconds().abs();
        difference <= 1
    }

    #[test]
    fn test_new_token_creation() {
        let now = Utc::now();

        let id = Uuid::new_v4();
        let subject = str();
        let audience = str();
        let issuer = str();
        let active_duration = now + Duration::hours(2);
        let active_from = now + Duration::minutes(12);
        let data = data_json();


        let issued_at = Utc::now();
        let token = UserSessionToken::new(
            id.clone(),
            subject.clone(),
            audience.clone(),
            issuer.clone(),
            active_duration.clone(),
            active_from.clone(),
            now,
            data.clone()
        );

        assert_eq!(token.id.get_version_num(), 4);
        assert_eq!(token.id, *token.get_id());

        assert_eq!(token.subject, subject);
        assert_eq!(token.get_subject(), subject);

        assert_eq!(token.audience, audience);
        assert_eq!(token.get_audience(), audience);

        assert_eq!(token.issuer, issuer);
        assert_eq!(token.get_issuer(), issuer);

        assert_eq!(active_duration, token.expiration.into());
        assert_eq!(*token.get_expiration(), active_duration);

        assert_eq!(active_from, token.not_before.into());
        assert_eq!(*token.get_not_before(), active_from);

        assert!(is_within_second(token.issued_at, issued_at));
        assert!(is_within_second(*token.get_issued_at(), issued_at));

        assert_eq!(token.custom_claims, data);
        assert_eq!(*token.get_custom_claims(), data)
    }
}