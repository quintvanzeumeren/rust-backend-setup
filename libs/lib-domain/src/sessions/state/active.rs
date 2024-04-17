use lib_auth::security::token::local_paseto_v4_token::LocalPasetoV4Token;
use crate::sessions::state::state::{State};
use crate::sessions::tokens::RefreshToken;

pub struct Active {
    pub latest_refresh_token: LocalPasetoV4Token<RefreshToken>,
}
impl State for Active {}