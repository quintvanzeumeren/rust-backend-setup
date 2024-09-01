
use crate::sessions::state::state::State;
use crate::sessions::user_session_token::UserSessionToken;
use crate::sessions::tokens::{AccessToken, RefreshToken};

pub struct NewlyCreated {
    pub(in crate::sessions) refresh_token: UserSessionToken<RefreshToken>,
    pub(in crate::sessions) access_token: UserSessionToken<AccessToken>,
}

impl NewlyCreated {
    pub fn refresh_token(&self) -> &UserSessionToken<RefreshToken> {
        &self.refresh_token
    }
    pub fn access_token(&self) -> &UserSessionToken<AccessToken> {
        &self.access_token
    }
}

impl State for NewlyCreated {}