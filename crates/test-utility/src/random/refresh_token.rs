use uuid::Uuid;
use security::token::token::Token;
use domain::sessions::user_session_token::UserSessionToken;
use domain::sessions::tokens::RefreshToken;
use domain::user::user_id::UserId;

pub fn random_refresh_token(user_id: UserId, session_id: &Uuid) -> UserSessionToken<RefreshToken> {
    RefreshToken {
        user_id: user_id.0,
        session_id: session_id.clone(),
        parent_id: None,
    }.into()
}

pub fn random_refresh_token_from(token: &UserSessionToken<RefreshToken>) -> UserSessionToken<RefreshToken> {
    RefreshToken {
        user_id: token.custom_claims.user_id.clone(),
        session_id: token.custom_claims.session_id.clone(),
        parent_id: Some(token.get_id().clone()),
    }.into()
}