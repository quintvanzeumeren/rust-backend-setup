use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use lib_auth::security::token::token::{Token};
use crate::sessions::user_session_token::UserSessionToken;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccessToken {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid,
}

impl Into<UserSessionToken<AccessToken>> for AccessToken {
    fn into(self) -> UserSessionToken<AccessToken> {

        let now = Utc::now();

        UserSessionToken::new(
            Uuid::new_v4(),
            "access_token".to_string(),
            self.user_id.to_string(),
            "rust_backend_setup".to_string(),
            now + Duration::minutes(5),
            now.clone(),
            now,
            self,
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub parent_id: Option<Uuid>,
}

impl Into<UserSessionToken<RefreshToken>> for RefreshToken {
    fn into(self) -> UserSessionToken<RefreshToken> {
        let now = Utc::now();

        UserSessionToken::new(
            Uuid::new_v4(),
            RefreshToken::subject().to_string(),
            self.user_id.to_string(),
            RefreshToken::issuer().to_string(),
            now + Duration::hours(4),
            now,
            now,
            self,
        )
    }
}

impl RefreshToken {
    pub fn subject() -> &'static str {
        "refresh_token"
    }

    pub fn issuer() -> &'static str {
        "rust_backend_setup"
    }
}
