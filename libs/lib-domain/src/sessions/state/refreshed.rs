
use crate::sessions::state::state::State;
use crate::sessions::user_session_token::UserSessionToken;
use crate::sessions::tokens::{AccessToken, RefreshToken};

pub struct Refreshed {
    pub(in crate::sessions) new_access_token: UserSessionToken<AccessToken>,
    pub(in crate::sessions) new_refresh_token: UserSessionToken<RefreshToken>,
    pub(in crate::sessions) old_refresh_token: UserSessionToken<RefreshToken>,
}

impl Refreshed {
    pub fn new_access_token(&self) -> &UserSessionToken<AccessToken> {
        &self.new_access_token
    }
    pub fn new_refresh_token(&self) -> &UserSessionToken<RefreshToken> {
        &self.new_refresh_token
    }
    pub fn old_refresh_token(&self) -> &UserSessionToken<RefreshToken> {
        &self.old_refresh_token
    }
}

impl State for Refreshed {}