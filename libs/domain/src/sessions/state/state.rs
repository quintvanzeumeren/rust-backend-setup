use crate::sessions::user_session_token::UserSessionToken;
use crate::sessions::tokens::RefreshToken;
pub trait State: Sized {}

#[derive(Debug, Clone)]
pub enum SessionEndReason {
    /// User ended the session by logging out.
    UserLogout,

    /// UserSignedInOnOtherDevice A user can have only active session at the time.
    /// If logged in onto another device, it will any other sessions.
    UserSignedInOnOtherDevice,

    /// Expired happens when the expiration date and time of the latest refresh token has past.
    LatestRefreshTokenExpired,

    /// AttemptedToReuseRefreshToken happens when a previously used refresh token
    /// is attempted to be used twice. May happen if the refresh token got stolen/leaked.
    AttemptedToReuseRefreshToken {
        caused_by: UserSessionToken<RefreshToken>
    },

    /// Similar reason for UsedExpiredRefreshToken. When access the access token is used after
    /// expiration date, it will very likely be malicious activity.
    UsedExpiredAccessToken,
}

impl SessionEndReason {
    pub fn to_string(&self) -> &str {
        match self {
            SessionEndReason::UserLogout => "UserLogout",
            SessionEndReason::UserSignedInOnOtherDevice => "UserSignedInOnOtherDevice",
            SessionEndReason::LatestRefreshTokenExpired => "LatestRefreshTokenExpired",
            SessionEndReason::AttemptedToReuseRefreshToken { .. } => "AttemptedToReuseRefreshToken",
            SessionEndReason::UsedExpiredAccessToken => "UsedExpiredAccessToken",
        }
    }
}