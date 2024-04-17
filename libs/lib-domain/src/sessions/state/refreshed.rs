use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use crate::sessions::state::state::State;
use crate::sessions::tokens::{AccessToken, RefreshToken};

pub struct Refreshed {
    pub(in crate::sessions) new_access_token: LocalPasetoV4Token<AccessToken>,
    pub(in crate::sessions) new_refresh_token: LocalPasetoV4Token<RefreshToken>,
    pub(in crate::sessions) old_refresh_token: LocalPasetoV4Token<RefreshToken>,
}

impl Refreshed {
    pub fn new_access_token(&self) -> &LocalPasetoV4Token<AccessToken> {
        &self.new_access_token
    }
    pub fn new_refresh_token(&self) -> &LocalPasetoV4Token<RefreshToken> {
        &self.new_refresh_token
    }
    pub fn old_refresh_token(&self) -> &LocalPasetoV4Token<RefreshToken> {
        &self.old_refresh_token
    }
}

impl State for Refreshed {}