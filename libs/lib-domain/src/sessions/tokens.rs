use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccessToken {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub refresh_token_id: Uuid,
}

impl Into<LocalPasetoV4Token<AccessToken>> for AccessToken {
    fn into(self) -> LocalPasetoV4Token<AccessToken> {
        LocalPasetoV4Token::new(
            "access_token",
            self.user_id.to_string().as_str(),
            "rust_backend_setup",
            Duration::minutes(5),
            Utc::now(),
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

impl Into<LocalPasetoV4Token<RefreshToken>> for RefreshToken {
    fn into(self) -> LocalPasetoV4Token<RefreshToken> {
        LocalPasetoV4Token::new(
            RefreshToken::subject(),
            self.user_id.to_string().as_str(),
            RefreshToken::issuer(),
            Duration::hours(4),
            Utc::now(),
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
