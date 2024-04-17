use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use lib_auth::security::token::token::Token;
use lib_domain::sessions::tokens::{AccessToken, RefreshToken};

pub fn random_access_token_for(refresh_token: LocalPasetoV4Token<RefreshToken>) -> LocalPasetoV4Token<AccessToken> {
    AccessToken {
        user_id: refresh_token.custom_claims.user_id.clone(),
        session_id: refresh_token.custom_claims.session_id.clone(),
        refresh_token_id: refresh_token.get_id().clone(),
    }.into()
}