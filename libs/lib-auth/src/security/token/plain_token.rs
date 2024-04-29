use chrono::{DateTime, Duration, Utc};
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;
use crate::security::token::token::Token;

// #[derive(Clone, Debug)]
// pub struct PlainToken<CustomClaims: Serialize + DeserializeOwned + Clone> {
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
// impl<CustomClaims: Serialize + DeserializeOwned + Clone> PlainToken<CustomClaims> {
//     pub fn new(
//         subject: &str,
//         audience: &str,
//         issuer: &str,
//         valid_duration: Duration,
//         active_from: DateTime<Utc>,
//         custom_claims: CustomClaims
//     ) -> PlainToken<CustomClaims> {
//         PlainToken {
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
// impl<'a, CustomClaims: Serialize + DeserializeOwned + Clone> Token<'a, CustomClaims> for PlainToken<CustomClaims> {
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
//     use serde::de::DeserializeOwned;
//     use serde::Serialize;
//     use serde_json::{json, Value};
//     use uuid::Uuid;
//     use crate::security::token::plain_token::PlainToken;
//     use crate::security::token::token::Token;
// 
//     fn str() -> String {
//         Uuid::new_v4().to_string()
//     }
// 
//     fn new_token<T: Serialize + DeserializeOwned + Clone>(data: T) -> PlainToken<T> {
//         PlainToken::new(
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
//         let token = PlainToken::new(
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
// }