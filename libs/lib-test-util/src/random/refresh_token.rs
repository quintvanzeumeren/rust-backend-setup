use uuid::Uuid;
use lib_auth::security::token::token::Token;
use lib_domain::sessions::user_session_token::UserSessionToken;
use lib_domain::sessions::tokens::RefreshToken;

pub fn random_refresh_token(user_id: &Uuid, session_id: &Uuid) -> UserSessionToken<RefreshToken> {
    RefreshToken {
        user_id: user_id.clone(),
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