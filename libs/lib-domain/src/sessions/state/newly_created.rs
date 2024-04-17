use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use crate::sessions::state::state::State;
use crate::sessions::tokens::{AccessToken, RefreshToken};

pub struct NewlyCreated {
    pub(in crate::sessions) refresh_token: LocalPasetoV4Token<RefreshToken>,
    pub(in crate::sessions) access_token: LocalPasetoV4Token<AccessToken>,
}

impl NewlyCreated {
    pub fn refresh_token(&self) -> &LocalPasetoV4Token<RefreshToken> {
        &self.refresh_token
    }
    pub fn access_token(&self) -> &LocalPasetoV4Token<AccessToken> {
        &self.access_token
    }
}

impl State for NewlyCreated {}