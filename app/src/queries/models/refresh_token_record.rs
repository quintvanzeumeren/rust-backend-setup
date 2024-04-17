use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use lib_auth::security::token::token::Token;
use lib_domain::sessions::tokens::RefreshToken;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RefreshTokenRecord {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub issued_at: NaiveDateTime,
    pub not_before: NaiveDateTime,
    pub expiration: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>
}

impl Into<LocalPasetoV4Token<RefreshToken>> for RefreshTokenRecord {
    fn into(self) -> LocalPasetoV4Token<RefreshToken> {
        LocalPasetoV4Token {
            id: self.id,
            subject: RefreshToken::subject().to_string(),
            audience: self.user_id.to_string(),
            issuer: RefreshToken::issuer().to_string(),
            expiration: DateTime::<Utc>::from_naive_utc_and_offset(self.expiration, Utc),
            not_before: DateTime::<Utc>::from_naive_utc_and_offset(self.not_before, Utc),
            issued_at: DateTime::<Utc>::from_naive_utc_and_offset(self.issued_at, Utc),
            custom_claims: RefreshToken {
                user_id: self.user_id,
                session_id: self.session_id,
                parent_id: self.parent_id,
            },
        }
    }
}
impl From<&LocalPasetoV4Token<RefreshToken>> for RefreshTokenRecord {
    fn from(token: &LocalPasetoV4Token<RefreshToken>) -> Self {
        RefreshTokenRecord {
            id: *token.get_id(),
            parent_id: token.get_custom_claims().parent_id,
            user_id: token.get_custom_claims().user_id,
            session_id: token.get_custom_claims().session_id,
            issued_at: token.get_issued_at().naive_utc(),
            not_before: token.get_not_before().naive_utc(),
            expiration: token.get_expiration().naive_utc(),
            used_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
    use lib_auth::security::token::token::Token;
    use lib_domain::sessions::tokens::RefreshToken;
    use lib_test_util::random::refresh_token::random_refresh_token;
    use lib_test_util::random::user_session::random_newly_created_user_session;
    use crate::queries::models::refresh_token_record::RefreshTokenRecord;

    #[test]
    fn test_from() {
        let user_id = Uuid::new_v4();
        let session = random_newly_created_user_session(&user_id);
        let token = random_refresh_token(&user_id, &session.user_id());

        let expected = RefreshTokenRecord {
            id: token.id,
            parent_id: None,
            user_id,
            session_id: token.get_custom_claims().session_id,
            issued_at: token.issued_at.naive_utc(),
            not_before: token.not_before.naive_utc(),
            expiration: token.expiration.naive_utc(),
            used_at: None,
        };

        let got = RefreshTokenRecord::from(&token);
        assert_eq!(expected, got)
    }
    
    #[test]
    fn test_into() {
        let user_id = Uuid::new_v4();
        let session = random_newly_created_user_session(&user_id);
        let token = random_refresh_token(&user_id, &session.user_id());

        let expected = RefreshTokenRecord::from(&token);
        let into: LocalPasetoV4Token<RefreshToken> = expected.clone().into();
        let got = RefreshTokenRecord::from(&into);
        assert_eq!(expected, got)
    }
}