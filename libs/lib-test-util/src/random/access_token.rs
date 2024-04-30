
use lib_auth::security::token::token::Token;
use lib_domain::sessions::user_session_token::UserSessionToken;
use lib_domain::sessions::tokens::{AccessToken, RefreshToken};

pub fn random_access_token_for(refresh_token: UserSessionToken<RefreshToken>) -> UserSessionToken<AccessToken> {
    AccessToken {
        user_id: refresh_token.custom_claims.user_id.clone(),
        session_id: refresh_token.custom_claims.session_id.clone(),
        refresh_token_id: refresh_token.get_id().clone(),
    }.into()
}