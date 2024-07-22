
use crate::sessions::state::state::{State};
use crate::sessions::user_session_token::UserSessionToken;
use crate::sessions::tokens::RefreshToken;

pub struct Active {
    pub latest_refresh_token: UserSessionToken<RefreshToken>,
}
impl State for Active {}